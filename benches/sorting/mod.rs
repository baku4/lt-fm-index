use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};
use lt_fm_index::{
    Text,
};

/*
Bench Sorting
*/

// Copy of 0.5.3

// Type 1: use 'libdivsufsort'
#[cfg(not(target_arch = "wasm32"))]
use libdivsufsort_rs::{divsufsort64, bw_transform64};
#[cfg(not(target_arch = "wasm32"))]
#[inline]
fn get_suffix_array_and_pidx_while_bwt_not_for_wasm(text: &mut Text) -> (Vec<i64>, u64) {
    let suffix_array_i64 = divsufsort64(text).unwrap();
    let pidx = {
        let mut sa = suffix_array_i64.clone();
        let pidx = bw_transform64(text, &mut sa).unwrap();
        pidx
    };

    (suffix_array_i64, pidx as u64)
}

// Type 2: use 'crate bio'
// Built-in Burrow Wheeler Transform Function
// For the environment that does not support building `libdivsufsort_rs`
use bio::data_structures::suffix_array::suffix_array as get_suffix_array;
use bio::data_structures::bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

#[inline]
fn get_suffix_array_and_pidx_while_bwt_for_wasm(text: &mut Text) -> (Vec<i64>, u64) {
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

use lt_fm_index::tests::random_text::{
    rand_text_with_length,
    UTF8_OF_AN,
};

pub fn bench_burrow_wheeler_transform(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("bw_transform");
    group.plot_config(plot_config);

    let text_lengths: Vec<usize> = (2..=7_u32).map(|x| {
        10_usize.pow(x)
    }).collect();

    for text_len in text_lengths {
        let mut text_1 = rand_text_with_length(&UTF8_OF_AN, text_len);
        let mut text_2 = text_1.clone();
        
        group.bench_with_input(
            BenchmarkId::new("livdivsufsort", text_len),
            &text_len,
            |b, _i| b.iter(|| {
                get_suffix_array_and_pidx_while_bwt_not_for_wasm(black_box(&mut text_1));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("crate_bio", text_len),
            &text_len,
            |b, _i| b.iter(|| {
                get_suffix_array_and_pidx_while_bwt_for_wasm(black_box(&mut text_2));
            }
        ));
    }
    group.finish();
}
