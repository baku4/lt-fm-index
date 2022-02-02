use criterion::{
    criterion_group, criterion_main, Criterion,
};

// Deprecated versions
mod unarchived_vs_dep;
mod archived_vs_unarchived;

mod counting_bit;

// Bench performance version of new vs deprecated
use unarchived_vs_dep::{
    bench_generate_unarchived_new_vs_dep,
    bench_locate_unarchived_new_vs_dep,
};

// Bench Archived vs Unarchived
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
use counting_bit::bench_counting_bits_of_u64;


// Profiler
mod profiler;
use profiler::FlamegraphProfiler;

fn custom_profiler() -> Criterion {
    Criterion::default().with_profiler(FlamegraphProfiler::new(100))
}

criterion_group!(
    name = benches;
    config = custom_profiler();
    targets = bench_load_casting_vs_including,
);
criterion_main!(benches);
