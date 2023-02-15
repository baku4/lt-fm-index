use crate::{TextLen, Text};
use bio::data_structures::suffix_array::suffix_array as get_suffix_array;
use bio::data_structures::bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

#[inline]
pub fn get_compressed_suffix_array_and_pidx_while_bwt_with_crate_bio(text: &mut Text, sampling_ratio: TextLen) -> (Vec<TextLen>, TextLen) {
    let mut input_string = text.to_vec();
    input_string.push(SENTINEL_SYMBOL);
    let mut suffix_array = get_suffix_array(&input_string);
    let mut bwt = get_bwt(&input_string, &suffix_array);
    
    let pidx = get_pidx_from_bwt(&bwt);

    bwt.remove(pidx);
    suffix_array.remove(0);

    // Change original text to bwt
    *text = bwt;
    let compressed_suffix_array = suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as TextLen).collect();
    (compressed_suffix_array, pidx as TextLen)
}

fn get_pidx_from_bwt(bwt: &[u8]) -> usize {
    for (index, &character) in bwt.iter().enumerate() {
        if character == SENTINEL_SYMBOL {
            return index
        }
    }
    0
}