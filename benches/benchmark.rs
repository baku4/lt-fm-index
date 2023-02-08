#![allow(dead_code)]
#![allow(unused_imports)]
use criterion::{
    criterion_group, criterion_main, Criterion,
};

// Bench counting bits
mod counting_bit;
use counting_bit::bench_counting_bits_of_u64;

// Sort algorithm
#[cfg(feature = "fastbwt")]
mod sorting;
#[cfg(feature = "fastbwt")]
use sorting::bench_burrow_wheeler_transform;

mod compare_perf;
use compare_perf::{
    build_no_text,
    locate_no_text,
};

#[cfg(not(feature = "fastbwt"))]
criterion_group!(
    benches,
    locate_no_text,
);
#[cfg(feature = "fastbwt")]
criterion_group!(
    benches,
    bench_burrow_wheeler_transform,
);
criterion_main!(benches);
