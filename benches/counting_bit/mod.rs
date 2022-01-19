use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId
};

/*
Bench bit counting
*/

pub fn bench_counting_bits_of_u64(c: &mut Criterion) {
    let bits_to_count: Vec<u64> = (0..=8_u32).map(|x| {
        2_u64.pow(x) - 1
    }).collect();

    let mut group = c.benchmark_group("counting_bits_of_u64");

    let bit_counts: Vec<usize> = (0..8).collect();

    for i in bit_counts {
        let bit_to_count = bits_to_count[i];
        
        group.bench_with_input(
            BenchmarkId::new("with_func", i),
            &i,
            |b, i| b.iter(|| {
                count_with_count_bits_method(black_box(bit_to_count));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("table_mut_var", i),
            &i,
            |b, i| b.iter(|| {
                count_with_bit_count_table_8_mut_var(black_box(bit_to_count));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("table_map", i),
            &i,
            |b, i| b.iter(|| {
                count_with_bit_count_table_8_map(black_box(bit_to_count));
            }
        ));
    }
    group.finish();
}

#[inline]
pub fn count_with_count_bits_method(bits_to_count: u64) {
    let rank = bits_to_count.count_ones();
}
#[inline]
pub fn count_with_bit_count_table_8_mut_var(bits_to_count: u64) {
    let mut rank = 0;
    bits_to_count.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE_8[byte as usize]);
}
#[inline]
pub fn count_with_bit_count_table_8_map(bits_to_count: u64) {
    let rank: u64 = bits_to_count.to_ne_bytes().iter()
        .map(|&byte| BIT_COUNT_TABLE_8[byte as usize]).sum::<u64>();
}
pub const BIT_COUNT_TABLE_8: [u64; 256] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];