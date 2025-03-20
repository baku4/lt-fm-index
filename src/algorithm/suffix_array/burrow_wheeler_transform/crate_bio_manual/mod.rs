use crate::core::Position;
use suffix_array::suffix_array as get_suffix_array;
use bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

#[inline]
pub fn get_compressed_suffix_array_and_pidx_while_bwt_with_crate_bio<P: Position>(text: &mut Vec<u8>, sampling_ratio: P) -> (Vec<P>, P) {
    let mut input_string = text.to_vec();
    input_string.push(SENTINEL_SYMBOL);
    let mut suffix_array = get_suffix_array(&input_string);
    let mut bwt = get_bwt(&input_string, &suffix_array);
    
    let pidx = get_pidx_from_bwt(&bwt);

    bwt.remove(pidx);
    suffix_array.remove(0);

    // Change original text to bwt
    *text = bwt;
    let compressed_suffix_array = suffix_array.into_iter().step_by(sampling_ratio.as_usize()).map(|x| P::from_usize(x)).collect();
    (compressed_suffix_array, P::from_usize(pidx))
}

fn get_pidx_from_bwt(bwt: &[u8]) -> usize {
    for (index, &character) in bwt.iter().enumerate() {
        if character == SENTINEL_SYMBOL {
            return index
        }
    }
    0
}

// =================================================================================================
// These modules are sourced from the Rust-Bio crate (https://github.com/rust-bio/rust-bio)
// version 2.2.0 and have been modified under the MIT License. Both the original code and
// the modifications remain under the MIT License. The original copyright
// and license notices are preserved at the top of each module.\
mod bwt;
mod alphabets;
mod suffix_array;
mod smallints;