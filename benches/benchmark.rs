use criterion::{
    criterion_group, criterion_main,
};

mod vs_dep;
mod counting_bit;

// Bench performance version of new vs deprecated
use vs_dep::{
    bench_generate_new_vs_dep,
    bench_locate_new_vs_dep,
};

// Bench counting bits
use counting_bit::bench_counting_bits_of_u64;

criterion_group!(
    benches,
    bench_generate_new_vs_dep,
    bench_locate_new_vs_dep,
);
criterion_main!(benches);