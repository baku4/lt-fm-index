use criterion::{
    criterion_group, criterion_main,
};

mod unarchived_vs_dep;
mod counting_bit;
mod serializer;

// Bench performance version of new vs deprecated
use unarchived_vs_dep::{
    bench_generate_unarchived_new_vs_dep,
    bench_locate_unarchived_new_vs_dep,
};

// Bench counting bits
use counting_bit::bench_counting_bits_of_u64;

// Bench serializer 
use serializer::{
    bench_serialization_btw_serializer,
};

criterion_group!(
    benches,
    bench_serialization_btw_serializer,
);
criterion_main!(benches);