use crate::algorithm::{LtFmIndex, CountArrayInterface, BwtInterface};
use crate::algorithm::count_array::CountArray;
use crate::algorithm::bwt::{Bwt, BwtBlockInterface};

use libdivsufsort_rs::{divsufsort64, bw_transform64};

type Text = Vec<u8>;

struct LtFmIndexBuilder {
    sa_sampling_ratio: u64,
    kmer_size: Option<usize>,
    bitcount_size: BitCountSize,
    countarray_builder: CountArrayBuilder,
    bwt_builder: BwtBuilder,
}

impl LtFmIndexBuilder {
    fn build<C: CountArrayInterface, B: BwtInterface>(self, text: Text) -> LtFmIndex<C, B> {
        let text_len = text.len() as u64;
        // (1) count array
        let count_array: C = CountArrayBuilder::build();
        // (2) suffix_array
        let suffix_array = divsufsort64(&text).unwrap();
        // (3) bwt & primary index
        let pidx = {
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut text, &mut sa).unwrap();
            pidx
        }; // original text is trasformed to bwt string
        // (4) compression
        let suffix_array = Self::compress_suffix_array(suffix_array, self.sa_sampling_ratio);
        let bwt: B = BwtBuilder::build();
        LtFmIndex {
            text_len,
            sa_sampling_ratio: self.sa_sampling_ratio,
            suffix_array,
            count_array,
            bwt,
        }
    }
    fn compress_suffix_array(suffix_array: Vec<i64>, sa_sampling_ratio: u64) -> Vec<u64> {

    }
}

struct CountArrayBuilder {
    
}

impl CountArrayBuilder {
    fn build<F>() -> CountArray<F> where F: Fn(u8) -> usize {

    }
}

struct BwtBuilder {

}

impl BwtBuilder {
    fn build<B: BwtBlockInterface>() -> Bwt<B> {

    }
}

enum BitCountSize {
    Bit8,
    Bit16,
}