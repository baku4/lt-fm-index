// Copyright 2014-2016 Johannes Köster, Taylor Cramer.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.

//! The [Burrows-Wheeler-Transform](https://www.semanticscholar.org/paper/A-Block-sorting-Lossless-Data-Compression-Algorithm-Burrows-Wheeler/af56e6d4901dcd0f589bf969e604663d40f1be5d) and related data structures.
//! The implementation is based on the lecture notes
//! "Algorithmen auf Sequenzen", Kopczynski, Marschall, Martin and Rahmann, 2008 - 2015.

use std::iter::repeat;

use super::suffix_array::RawSuffixArraySlice;

pub type BWT = Vec<u8>;
pub type Less = Vec<usize>;

pub fn bwt(text: &[u8], pos: RawSuffixArraySlice) -> BWT {
    assert_eq!(text.len(), pos.len());
    let n = text.len();
    let mut bwt: BWT = repeat(0).take(n).collect();
    for r in 0..n {
        let p = pos[r];
        bwt[r] = if p > 0 { text[p - 1] } else { text[n - 1] };
    }

    bwt
}


#[derive(Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Occ {
    occ: Vec<Vec<usize>>,
    k: u32,
}
