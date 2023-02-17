use crate_fm_index::converter::RangeConverter;
use crate_fm_index::suffix_array::{SuffixOrderSampler, SuffixOrderSampledArray};
use crate_fm_index::{BackwardSearchIndex, FMIndex};

type OtherFmIndex = FMIndex<u8, RangeConverter<u8>, SuffixOrderSampledArray>;

pub fn get_fmindex_of_other_crate(text: &Vec<u8>) -> OtherFmIndex {
    let converter = RangeConverter::new(b' ', b'~');
    let sampler = SuffixOrderSampler::new().level(2);
    let index = FMIndex::new(text.clone(), converter, sampler);
    index
}

pub fn get_sorted_locations(fm_index: &OtherFmIndex, pattern: &Vec<u8>) -> Vec<u64> {
    let search = fm_index.search_backward(pattern);
    let mut location = search.locate();
    location.sort();
    location
}
