// Copyright 2014-2016 Johannes Köster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::iter::Enumerate;
use std::mem::size_of;
use std::slice;

use num_integer::Integer;
use num_traits::{cast, Bounded, Num, NumCast};

/// Data structure for storing a sequence of small integers with few big ones space efficiently
/// while supporting classical vector operations.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SmallInts<F: Integer + Bounded + NumCast + Copy, B: Integer + NumCast + Copy> {
    smallints: Vec<F>,
    bigints: BTreeMap<usize, B>,
}

impl<S: Integer + Bounded + NumCast + Copy, B: Integer + NumCast + Copy> Default
    for SmallInts<S, B>
{
    fn default() -> Self {
        assert!(
            size_of::<S>() < size_of::<B>(),
            "S has to be smaller than B"
        );
        SmallInts {
            smallints: Vec::new(),
            bigints: BTreeMap::new(),
        }
    }
}

impl<S: Integer + Bounded + NumCast + Copy, B: Integer + NumCast + Copy> SmallInts<S, B> {
    fn real_value(&self, i: usize, v: S) -> Option<B> {
        if v < S::max_value() {
            cast(v)
        } else {
            self.bigints.get(&i).cloned()
        }
    }
}

/// Iterator over the elements of a `SmallInts` sequence.
#[derive(Clone, Debug)]
pub struct Iter<'a, S, B>
where
    S: Integer + Bounded + NumCast + Copy,
    B: Integer + NumCast + Copy,
    <S as Num>::FromStrRadixErr: 'a,
    <B as Num>::FromStrRadixErr: 'a,
{
    smallints: &'a SmallInts<S, B>,
    items: Enumerate<slice::Iter<'a, S>>,
}

impl<'a, S, B> Iterator for Iter<'a, S, B>
where
    S: 'a + Integer + Bounded + NumCast + Copy,
    B: 'a + Integer + NumCast + Copy,
    <S as Num>::FromStrRadixErr: 'a,
    <B as Num>::FromStrRadixErr: 'a,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        match self.items.next() {
            Some((i, &v)) => self.smallints.real_value(i, v),
            None => None,
        }
    }
}
