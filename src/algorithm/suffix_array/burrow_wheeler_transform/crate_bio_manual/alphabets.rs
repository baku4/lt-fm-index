// Copyright 2014-2015 Johannes Köster, Peer Aramillo Irizar.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Borrow;

use bit_set::BitSet;
use vec_map::VecMap;

pub type SymbolRanks = VecMap<u8>;

/// Representation of an alphabet.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Alphabet {
    pub symbols: BitSet,
}

impl Alphabet {
    pub fn new<C, T>(symbols: T) -> Self
    where
        C: Borrow<u8>,
        T: IntoIterator<Item = C>,
    {
        let mut s = BitSet::new();
        s.extend(symbols.into_iter().map(|c| *c.borrow() as usize));

        Alphabet { symbols: s }
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }
}

/// Tools based on transforming the alphabet symbols to their lexicographical ranks.
///
/// Lexicographical rank is computed using `u8` representations,
/// i.e. ASCII codes, of the input characters.
/// For example, assuming that the alphabet consists of the symbols `A`, `C`, `G`, and `T`, this
/// will yield ranks `0`, `1`, `2`, `3` for them, respectively.
///
/// `RankTransform` can be used in to perform bit encoding for texts over a
/// given alphabet via `bio::data_structures::bitenc`.
#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RankTransform {
    pub ranks: SymbolRanks,
}

impl RankTransform {
    pub fn new(alphabet: &Alphabet) -> Self {
        let mut ranks = VecMap::new();
        for (r, c) in alphabet.symbols.iter().enumerate() {
            ranks.insert(c, r as u8);
        }

        RankTransform { ranks }
    }

    #[inline]
    pub fn get(&self, a: u8) -> u8 {
        *self.ranks.get(a as usize).expect("Unexpected character.")
    }
}

/// Iterator over q-grams.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct QGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: Iterator<Item = C>,
{
    text: T,
    ranks: &'a RankTransform,
    q: u32,
    bits: u32,
    mask: usize,
    qgram: usize,
}

impl<'a, C, T> QGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: Iterator<Item = C>,
{
    /// Push a new character into the current qgram.
    #[inline]
    fn qgram_push(&mut self, a: u8) {
        self.qgram <<= self.bits;
        self.qgram |= a as usize;
        self.qgram &= self.mask;
    }
}

impl<'a, C, T> Iterator for QGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: Iterator<Item = C>,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        match self.text.next() {
            Some(a) => {
                let b = self.ranks.get(*a.borrow());
                self.qgram_push(b);
                Some(self.qgram)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.text.size_hint()
    }
}

impl<'a, C, T> ExactSizeIterator for QGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: ExactSizeIterator<Item = C>,
{
}

/// Iterator over q-grams.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct RevQGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: DoubleEndedIterator<Item = C>,
{
    text: T,
    ranks: &'a RankTransform,
    q: u32,
    bits: u32,
    left_shift: u32,
    qgram: usize,
}

impl<'a, C, T> RevQGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: DoubleEndedIterator<Item = C>,
{
    /// Push a new character into the current qgram in the reverse direction.
    #[inline]
    fn qgram_push_rev(&mut self, a: u8) {
        self.qgram >>= self.bits;
        self.qgram |= (a as usize) << self.left_shift;
    }
}

impl<'a, C, T> Iterator for RevQGrams<'a, C, T>
where
    C: Borrow<u8>,
    T: DoubleEndedIterator<Item = C>,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        match self.text.next_back() {
            Some(a) => {
                let b = self.ranks.get(*a.borrow());
                self.qgram_push_rev(b);
                Some(self.qgram)
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.text.size_hint()
    }
}
