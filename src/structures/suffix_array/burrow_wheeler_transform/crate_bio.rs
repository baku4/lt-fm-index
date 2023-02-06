use super::Text;
use bio::data_structures::suffix_array::suffix_array as get_suffix_array;
use bio::data_structures::bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

#[inline]
pub fn get_suffix_array_and_pidx_while_bwt_with_crate_bio(text: &mut Text) -> (Vec<i64>, u64) {
    let mut input_string = text.to_vec();
    input_string.push(SENTINEL_SYMBOL);
    let mut suffix_array = get_suffix_array(&input_string);
    let mut bwt = get_bwt(&input_string, &suffix_array);
    
    let pidx = get_pidx_from_bwt(&bwt);

    bwt.remove(pidx);
    suffix_array.remove(0);

    // Change original text to bwt
    *text = bwt;

    (suffix_array.into_iter().map(|v| v as i64).collect(), pidx as u64)
}

fn get_pidx_from_bwt(bwt: &[u8]) -> usize {
    for (index, &character) in bwt.iter().enumerate() {
        if character == SENTINEL_SYMBOL {
            return index
        }
    }
    0
}