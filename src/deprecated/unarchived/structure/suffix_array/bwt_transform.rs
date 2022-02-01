// Built-in Burrow Wheeler Transform Function
// For the environment that does not support building `libdivsufsort_rs`
use bio::data_structures::suffix_array::suffix_array as get_suffix_array;
use bio::data_structures::bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

pub fn burrow_wheeler_transform(input_string: &[u8]) -> (Vec<i64>, Vec<u8>, u64) { // TODO:
    let mut input_string = input_string.to_vec();
    input_string.push(SENTINEL_SYMBOL);
    let mut suffix_array = get_suffix_array(&input_string);
    let mut bwt = get_bwt(&input_string, &suffix_array);
    
    let pidx = get_pidx_from_bwt(&bwt);

    bwt.remove(pidx);
    suffix_array.remove(0);

    (suffix_array.into_iter().map(|v| v as i64).collect(), bwt, pidx as u64)
}

fn get_pidx_from_bwt(bwt: &[u8]) -> usize {
    for (index, &character) in bwt.iter().enumerate() {
        if character == SENTINEL_SYMBOL {
            return index
        }
    }
    0
}

#[test]
fn print_bwt_test() {
    
}