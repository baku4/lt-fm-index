use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};
use lt_fm_index::{
    LtFmIndex, blocks::{
        Block2, Block3, Block4,
    }, Position, Block,
};
use super::random_data::{
    gen_rand_text,
    gen_rand_pattern,
};
use std::time::{Duration, Instant};

type Lfi32_2_32 = LtFmIndex<u32, Block2<u32>>;
type Lfi32_2_64 = LtFmIndex<u32, Block2<u64>>;
type Lfi32_2_128 = LtFmIndex<u32, Block2<u128>>;
type Lfi32_3_32 = LtFmIndex<u32, Block3<u32>>;
type Lfi32_3_64 = LtFmIndex<u32, Block3<u64>>;
type Lfi32_3_128 = LtFmIndex<u32, Block3<u128>>;
type Lfi32_4_32 = LtFmIndex<u32, Block4<u32>>;
type Lfi32_4_64 = LtFmIndex<u32, Block4<u64>>;
type Lfi32_4_128 = LtFmIndex<u32, Block4<u128>>;
type Lfi64_2_32 = LtFmIndex<u64, Block2<u32>>;
type Lfi64_2_64 = LtFmIndex<u64, Block2<u64>>;
type Lfi64_2_128 = LtFmIndex<u64, Block2<u128>>;
type Lfi64_3_32 = LtFmIndex<u64, Block3<u32>>;
type Lfi64_3_64 = LtFmIndex<u64, Block3<u64>>;
type Lfi64_3_128 = LtFmIndex<u64, Block3<u128>>;
type Lfi64_4_32 = LtFmIndex<u64, Block4<u32>>;
type Lfi64_4_64 = LtFmIndex<u64, Block4<u64>>;
type Lfi64_4_128 = LtFmIndex<u64, Block4<u128>>;

#[inline]
fn locate_multiple_patterns<P: Position, B: Block<P>>(
    lfi: &LtFmIndex<P, B>,
    patterns: &[Vec<u8>]
) {
    patterns.iter().for_each(|pattern| {
        _ = lfi.locate(pattern);
    });
}

#[inline]
fn locate_multiple_patterns_from_raw_index<P: Position, B: Block<P>>(
    lfi: &LtFmIndex<P, B>,
    patterns: &[Vec<u8>]
) {
    patterns.iter().for_each(|pattern| {
        let decoding_table = lfi.decoding_table();
        let raw_index_rev_iter = pattern.iter().map(|&c| decoding_table[c as usize]).rev();
        _ = lfi.locate_from_raw_index(raw_index_rev_iter);
    });
}

pub fn compare_locate_vs_locate_from_raw_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("locate_vs_locate_from_raw_index");

    let text_len = 10_000;
    let text = gen_rand_text(b"ACG", text_len, text_len);

    let n_patterns = 100;
    let pattern_length = [50]; // [10, 20, 30, 40, 50];
    let patterns_by_length: Vec<Vec<Vec<u8>>> = pattern_length.iter().map(|l| {
        let patterns = (0..n_patterns).map(|_| {
            gen_rand_pattern(&text, *l, *l)
        }).collect();
        patterns
    }).collect();   

    let ss_list = [4];
    let lk_list = [4];

    let characters_by_index: &[&[u8]] = &[b"A", b"C", b"G"];

    for ss in ss_list {
        for lk in lk_list {
            println!("# SS: {}, LK: {}", ss, lk);
            macro_rules! TestCode {
                ( $lfity: ident, $tagprefix: tt ) => {
                    {
                        let tag = format!("{}_ss{}_lk{}", $tagprefix, ss, lk);
                        let lfi = {
                            let start = Instant::now();
                            let lfi = $lfity::build(text.clone(), characters_by_index, ss as _, lk).unwrap();
                            let duration = start.elapsed();
                            println!(" - {}: built in {:?}s", tag, duration);
                            lfi
                        };

                        for (pattern_len, patterns) in pattern_length.iter().zip(patterns_by_length.iter()) {
                            group.bench_with_input(
                                BenchmarkId::new(format!("{}_locate", tag), pattern_len),
                                &pattern_len,
                                |b, _i| b.iter(|| {
                                    locate_multiple_patterns(
                                        black_box(&lfi),
                                        black_box(patterns),
                                    );
                                }
                            ));

                            group.bench_with_input(
                                BenchmarkId::new(format!("{}_locate_from_raw_index", tag), pattern_len),
                                &pattern_len,
                                |b, _i| b.iter(|| {
                                    locate_multiple_patterns_from_raw_index(
                                        black_box(&lfi),
                                        black_box(patterns),
                                    );
                                }
                            ));
                        }
                    }
                };
            }
            TestCode!(Lfi32_2_32, "LFI_u32_b2_v32");
            // TestCode!(Lfi32_2_64, "LFI_u32_b2_v64");
            // TestCode!(Lfi32_2_128, "LFI_u32_b2_v128");
            // TestCode!(Lfi32_4_32, "LFI_u32_b4_v32");
            TestCode!(Lfi32_4_64, "LFI_u32_b4_v64");
            // TestCode!(Lfi32_4_128, "LFI_u32_b4_v128");
            TestCode!(Lfi64_2_32, "LFI_u64_b2_v32");
            // TestCode!(Lfi64_2_64, "LFI_u64_b2_v64");
            // TestCode!(Lfi64_2_128, "LFI_u64_b2_v128");
            // TestCode!(Lfi64_4_32, "LFI_u64_b4_v32");
            TestCode!(Lfi64_4_64, "LFI_u64_b4_v64");
        }
    }

    group.finish();
}