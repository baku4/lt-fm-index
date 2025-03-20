#![allow(dead_code)]
#![allow(unused_imports)]
use criterion::{
    criterion_group, criterion_main, Criterion,
};

mod random_data;

// Bench counting bits
mod counting_bit;
use counting_bit::bench_counting_bits_of_u64;

// Sort algorithm
#[cfg(feature = "fastbwt")]
mod sorting;
#[cfg(feature = "fastbwt")]
use sorting::bench_burrow_wheeler_transform;

// Bench of locating by options
mod locate;
use locate::perf_of_locate;

mod locate_with_raw_index;
use locate_with_raw_index::compare_locate_vs_locate_from_raw_index;

// mod perf_by_vector_size;
// use perf_by_vector_size::{
//     build_no_text, locate_no_text,
// };

criterion_group!(
    benches,
    compare_locate_vs_locate_from_raw_index,
);
#[cfg(feature = "fastbwt")]
criterion_group!(
    benches,
    bench_burrow_wheeler_transform,
);
criterion_main!(benches);
