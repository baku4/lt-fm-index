#![allow(dead_code)]
#![allow(unused_imports)]
use criterion::{
    criterion_group, criterion_main, Criterion,
};

// Deprecated versions
// Bench Archived vs Unarchived
#[cfg(target_feature = "zero_copy")]
mod archived_vs_unarchived;
#[cfg(target_feature = "zero_copy")]
use archived_vs_unarchived::{
    bench_serialization_btw_serializer,
    bench_build_arc_vs_unarc,
    bench_save_arc_vs_unarc,
    bench_load_arc_vs_unarc,
    bench_locate_arc_vs_unarc,
    bench_save_taking_vs_writing,
    bench_load_casting_vs_including,
};

// Bench counting bits
mod counting_bit;
use counting_bit::bench_counting_bits_of_u64;

// Sort algorithm
mod sorting;
use sorting::bench_burrow_wheeler_transform;

criterion_group!(
    benches,
    bench_burrow_wheeler_transform,
);
criterion_main!(benches);
