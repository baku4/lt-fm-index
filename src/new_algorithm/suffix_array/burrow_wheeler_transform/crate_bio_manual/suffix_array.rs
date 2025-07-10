// Copyright 2014-2016 Johannes Köster, Taylor Cramer.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::BuildHasherDefault;

use num_integer::Integer;
use num_traits::{cast, NumCast, Unsigned};

use bv::{BitVec, Bits, BitsMut};
use vec_map::VecMap;

use fxhash::FxHasher;

use super::alphabets::{Alphabet, RankTransform};
use super::bwt::{Less, Occ, BWT};

pub type RawSuffixArray = Vec<usize>;
pub type RawSuffixArraySlice<'a> = &'a [usize];

type HashMapFx<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// A sampled suffix array.
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct SampledSuffixArray<DBWT: Borrow<BWT>, DLess: Borrow<Less>, DOcc: Borrow<Occ>> {
    bwt: DBWT,
    less: DLess,
    occ: DOcc,
    sample: Vec<usize>,
    s: usize, // Rate of sampling
    extra_rows: HashMapFx<usize, usize>,
    sentinel: u8,
}

pub fn suffix_array(text: &[u8]) -> RawSuffixArray {
    let n = text.len();
    let alphabet = Alphabet::new(text);
    let sentinel_count = sentinel_count(text);
    let mut sais = Sais::new(n);

    match alphabet.len() + sentinel_count {
        a if a <= std::u8::MAX as usize => {
            sais.construct(&transform_text::<u8>(text, &alphabet, sentinel_count))
        }
        a if a <= std::u16::MAX as usize => {
            sais.construct(&transform_text::<u16>(text, &alphabet, sentinel_count))
        }
        a if a <= std::u32::MAX as usize => {
            sais.construct(&transform_text::<u32>(text, &alphabet, sentinel_count))
        }
        _ => sais.construct(&transform_text::<u64>(text, &alphabet, sentinel_count)),
    }

    sais.pos
}

/// Return last character of the text (expected to be the sentinel).
fn sentinel(text: &[u8]) -> u8 {
    text[text.len() - 1]
}

/// Count the sentinels occurring in the text given that the last character is the sentinel.
fn sentinel_count(text: &[u8]) -> usize {
    let sentinel = sentinel(text);
    assert!(
        text.iter().all(|&a| a >= sentinel),
        "Expecting extra sentinel symbol being lexicographically smallest at the end of the \
         text."
    );

    text.iter()
        .fold(0, |count, &a| count + (a == sentinel) as usize)
}

/// Transform the given text into integers for usage in `SAIS`.
fn transform_text<T: Integer + Unsigned + NumCast + Copy + Debug>(
    text: &[u8],
    alphabet: &Alphabet,
    sentinel_count: usize,
) -> Vec<T> {
    let sentinel = sentinel(text);
    let transform = RankTransform::new(alphabet);
    let offset = sentinel_count - 1;

    let mut transformed: Vec<T> = Vec::with_capacity(text.len());
    let mut s = sentinel_count;
    for &a in text {
        if a == sentinel {
            s -= 1;
            transformed.push(cast(s).unwrap());
        } else {
            transformed
                .push(cast(*(transform.ranks.get(a as usize)).unwrap() as usize + offset).unwrap());
        }
    }

    transformed
}

/// SAIS implementation (see function `suffix_array` for description).
struct Sais {
    pos: Vec<usize>,
    lms_pos: Vec<usize>,
    reduced_text_pos: Vec<usize>,
    bucket_sizes: VecMap<usize>,
    bucket_start: Vec<usize>,
    bucket_end: Vec<usize>,
}

impl Sais {
    /// Create a new instance.
    fn new(n: usize) -> Self {
        Sais {
            pos: Vec::with_capacity(n),
            lms_pos: Vec::with_capacity(n),
            reduced_text_pos: vec![0; n],
            bucket_sizes: VecMap::new(),
            bucket_start: Vec::with_capacity(n),
            bucket_end: Vec::with_capacity(n),
        }
    }

    /// Init buckets.
    fn init_bucket_start<T: Integer + Unsigned + NumCast + Copy>(&mut self, text: &[T]) {
        self.bucket_sizes.clear();
        self.bucket_start.clear();

        for &c in text {
            if !self.bucket_sizes.contains_key(cast(c).unwrap()) {
                self.bucket_sizes.insert(cast(c).unwrap(), 0);
            }
            *(self.bucket_sizes.get_mut(cast(c).unwrap()).unwrap()) += 1;
        }

        let mut sum = 0;
        for &size in self.bucket_sizes.values() {
            self.bucket_start.push(sum);
            sum += size;
        }
    }

    /// Initialize pointers to the last element of the buckets.
    fn init_bucket_end<T: Integer + Unsigned + NumCast + Copy>(&mut self, text: &[T]) {
        self.bucket_end.clear();
        for &r in &self.bucket_start[1..] {
            self.bucket_end.push(r - 1);
        }
        self.bucket_end.push(text.len() - 1);
    }

    /// Check if two LMS substrings are equal.
    fn lms_substring_eq<T: Integer + Unsigned + NumCast + Copy>(
        &self,
        text: &[T],
        pos_types: &PosTypes,
        i: usize,
        j: usize,
    ) -> bool {
        for k in 0.. {
            let lmsi = pos_types.is_lms_pos(i + k);
            let lmsj = pos_types.is_lms_pos(j + k);
            if text[i + k] != text[j + k] {
                // different symbols
                return false;
            }
            if lmsi != lmsj {
                // different length
                return false;
            }
            if k > 0 && lmsi && lmsj {
                // same symbols and same length
                return true;
            }
        }
        false
    }

    /// Sort LMS suffixes.
    fn sort_lms_suffixes<
        T: Integer + Unsigned + NumCast + Copy + Debug,
        S: Integer + Unsigned + NumCast + Copy + Debug,
    >(
        &mut self,
        text: &[T],
        pos_types: &PosTypes,
        lms_substring_count: usize,
    ) {
        // if less than 2 LMS substrings are present, no further sorting is needed
        if lms_substring_count > 1 {
            // sort LMS suffixes by recursively building SA on reduced text
            let mut reduced_text: Vec<S> = vec![cast(0).unwrap(); lms_substring_count];
            let mut label = 0;
            reduced_text[self.reduced_text_pos[self.pos[0]]] = cast(label).unwrap();
            let mut prev = None;
            for &p in &self.pos {
                if pos_types.is_lms_pos(p) {
                    // choose same label if substrings are equal
                    if prev.is_some() && !self.lms_substring_eq(text, pos_types, prev.unwrap(), p) {
                        label += 1;
                    }
                    reduced_text[self.reduced_text_pos[p]] = cast(label).unwrap();
                    prev = Some(p);
                }
            }

            // if we have less labels than substrings, we have to sort by recursion
            // because two or more substrings are equal
            if label + 1 < lms_substring_count {
                // backup lms_pos
                let lms_pos = self.lms_pos.clone();
                // recurse SA construction for reduced text
                self.construct(&reduced_text);
                // obtain sorted lms suffixes
                self.lms_pos.clear();
                for &p in &self.pos {
                    self.lms_pos.push(lms_pos[p]);
                }
            } else {
                // otherwise, lms_pos is updated with the sorted suffixes from pos
                // obtain sorted lms suffixes
                self.lms_pos.clear();
                for &p in &self.pos {
                    if pos_types.is_lms_pos(p) {
                        self.lms_pos.push(p);
                    }
                }
            }
        }
    }

    /// Construct the suffix array.
    fn construct<T: Integer + Unsigned + NumCast + Copy + Debug>(&mut self, text: &[T]) {
        let pos_types = PosTypes::new(text);
        self.calc_lms_pos(text, &pos_types);
        self.calc_pos(text, &pos_types);
    }

    /// Step 1 of the SAIS algorithm.
    fn calc_lms_pos<T: Integer + Unsigned + NumCast + Copy + Debug>(
        &mut self,
        text: &[T],
        pos_types: &PosTypes,
    ) {
        let n = text.len();

        // collect LMS positions
        self.lms_pos.clear();
        let mut i = 0;
        for r in 0..n {
            if pos_types.is_lms_pos(r) {
                self.lms_pos.push(r);
                self.reduced_text_pos[r] = i;
                i += 1;
            }
        }

        // sort LMS substrings by applying step 2 with unsorted LMS positions
        self.calc_pos(text, pos_types);

        let lms_substring_count = self.lms_pos.len();

        if lms_substring_count <= std::u8::MAX as usize {
            self.sort_lms_suffixes::<T, u8>(text, pos_types, lms_substring_count);
        } else if lms_substring_count <= std::u16::MAX as usize {
            self.sort_lms_suffixes::<T, u16>(text, pos_types, lms_substring_count);
        } else if lms_substring_count <= std::u32::MAX as usize {
            self.sort_lms_suffixes::<T, u32>(text, pos_types, lms_substring_count);
        } else {
            self.sort_lms_suffixes::<T, u64>(text, pos_types, lms_substring_count);
        }
    }

    /// Step 2 of the SAIS algorithm.
    fn calc_pos<T: Integer + Unsigned + NumCast + Copy>(
        &mut self,
        text: &[T],
        pos_types: &PosTypes,
    ) {
        let n = text.len();
        self.pos.clear();

        self.init_bucket_start(text);
        self.init_bucket_end(text);

        // init all positions as unknown (n-1 is max position)
        self.pos.resize(n, n);

        // insert LMS positions to the end of their buckets
        for &p in self.lms_pos.iter().rev() {
            let c: usize = cast(text[p]).unwrap();
            self.pos[self.bucket_end[c]] = p;
            // subtract without overflow: last -1 will cause overflow, but it does not matter
            self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
        }

        // reset bucket ends
        self.init_bucket_end(text);

        // insert L-positions into buckets
        for r in 0..n {
            let p = self.pos[r];
            // ignore undefined positions and the zero since it has no predecessor
            if p == n || p == 0 {
                continue;
            }
            let pred = p - 1;
            if pos_types.is_l_pos(pred) {
                let c: usize = cast(text[pred]).unwrap();
                self.pos[self.bucket_start[c]] = pred;
                self.bucket_start[c] += 1;
            }
        }

        // insert S-positions into buckets
        for r in (0..n).rev() {
            let p = self.pos[r];
            if p == 0 {
                continue;
            }
            let pred = p - 1;
            if pos_types.is_s_pos(pred) {
                let c: usize = cast(text[pred]).unwrap();
                self.pos[self.bucket_end[c]] = pred;
                // subtract without overflow: last -1 will cause overflow, but it won't be used
                self.bucket_end[c] = self.bucket_end[c].wrapping_sub(1);
            }
        }
    }
}

/// Position types (L or S).
#[derive(Debug)]
struct PosTypes {
    pos_types: BitVec,
}

impl PosTypes {
    /// Calculate the text position type.
    /// L-type marks suffixes being lexicographically larger than their successor,
    /// S-type marks the others.
    /// This function fills a BitVec, with 1-bits denoting S-type
    /// and 0-bits denoting L-type.
    ///
    /// # Arguments
    ///
    /// * `text` - the text, ending with a sentinel.
    fn new<T: Integer + Unsigned + NumCast + Copy>(text: &[T]) -> Self {
        let n = text.len();
        let mut pos_types = BitVec::new_fill(false, n as u64);
        pos_types.set_bit(n as u64 - 1, true);

        for p in (0..n - 1).rev() {
            if text[p] == text[p + 1] {
                // if the characters are equal, the next position determines
                // the lexicographical order
                let v = pos_types.get_bit(p as u64 + 1);
                pos_types.set_bit(p as u64, v);
            } else {
                pos_types.set_bit(p as u64, text[p] < text[p + 1]);
            }
        }

        PosTypes { pos_types }
    }

    /// Check if p is S-position.
    fn is_s_pos(&self, p: usize) -> bool {
        self.pos_types.get_bit(p as u64)
    }

    /// Check if p is L-position.
    fn is_l_pos(&self, p: usize) -> bool {
        !self.pos_types.get_bit(p as u64)
    }

    /// Check if p is LMS-position.
    fn is_lms_pos(&self, p: usize) -> bool {
        p != 0 && self.is_s_pos(p) && self.is_l_pos(p - 1)
    }
}
