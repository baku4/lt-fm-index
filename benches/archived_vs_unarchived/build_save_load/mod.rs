use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::deprecated::archived::*;
use lt_fm_index::deprecated::unarchived::*;
use lt_fm_index::tests::random_text::*;

// ***************************************************
//
// Bench performance version of archived vs unarchived
//
// ***************************************************

// Functions for Build
const KMER_SIZE_NO: usize = 6;
const KMER_SIZE_AN: usize = 4;
const SAMPLING_RATIO: u64 = 2;

#[inline]
fn build_arc_64no(text: Vec<u8>) -> LtFmIndex {
    LtFmIndexBuilder::new()
        .use_nucleotide_only()
        .compress_bwt_64()
        .set_lookup_table_kmer_size(KMER_SIZE_NO).unwrap()
        .set_suffix_array_sampling_ratio(SAMPLING_RATIO).unwrap()
        .build(text)
}
#[inline]
fn build_arc_64an(text: Vec<u8>) -> LtFmIndex {
    LtFmIndexBuilder::new()
        .use_amino_acid_with_noise()
        .compress_bwt_64()
        .set_lookup_table_kmer_size(KMER_SIZE_AN).unwrap()
        .set_suffix_array_sampling_ratio(SAMPLING_RATIO).unwrap()
        .build(text)
}
#[inline]
fn build_unarc_64no(text: Vec<u8>) -> LtFmIndexAll {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(KMER_SIZE_NO).unwrap()
        .change_sampling_ratio(SAMPLING_RATIO).unwrap()
        .generate(text).unwrap()
}
#[inline]
fn build_unarc_64an(text: Vec<u8>) -> LtFmIndexAll {
    LtFmIndexConfig::for_aminoacid()
        .with_noise()
        .change_kmer_size(KMER_SIZE_AN).unwrap()
        .change_sampling_ratio(SAMPLING_RATIO).unwrap()
        .generate(text).unwrap()
}

// Functions for Save
#[inline]
fn save_arc(lt_fm_index: LtFmIndex) -> Vec<u8> {
    let mut buffer = Vec::new();
    let _ = lt_fm_index.save_to(&mut buffer);
    buffer
}
#[inline]
fn save_unarc(lt_fm_index: LtFmIndexAll) -> Vec<u8>{
    let mut buffer = Vec::new();
    let _ = lt_fm_index.write_to(&mut buffer);
    buffer
}

// Functions for Load
#[inline]
fn load_arc_checked(buffer: Vec<u8>) -> LtFmIndex {
    LtFmIndex::load_from(&buffer[..]).unwrap()
}
#[inline]
fn load_arc_unchecked(buffer: Vec<u8>) -> LtFmIndex {
    LtFmIndex::unchecked_load_from(&buffer[..]).unwrap()
}
#[inline]
fn load_unarc(buffer: Vec<u8>) -> LtFmIndexAll {
    LtFmIndexAll::read_from(&buffer[..]).unwrap()
}

// 1
// Build Bench
pub fn bench_build_arc_vs_unarc(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("build_arc_vs_unarc");

    group.plot_config(plot_config.clone());

    let text_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let text_an = rand_text_with_length(&UTF8_OF_AN, text_len);

        group.bench_with_input(
            BenchmarkId::new("no_arc", text_len),
            &text_len,
            |b, i| b.iter(|| {
                build_arc_64no(black_box(text_no.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("no_unarc", text_len),
            &text_len,
            |b, i| b.iter(|| {
                build_unarc_64no(black_box(text_no.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("an_arc", text_len),
            &text_len,
            |b, i| b.iter(|| {
                build_arc_64an(black_box(text_an.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("an_unarc", text_len),
            &text_len,
            |b, i| b.iter(|| {
                build_unarc_64an(black_box(text_an.clone()));
            }
        ));
    }
    group.finish();
}

// 2
// Save Bench
pub fn bench_save_arc_vs_unarc(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("save_arc_vs_unarc");

    group.plot_config(plot_config.clone());

    let text_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let text_an = rand_text_with_length(&UTF8_OF_AN, text_len);
        // NO_Arc
        {
            let lf_fm_index = build_arc_64no(text_no.clone());

            // Save
            group.bench_with_input(
                BenchmarkId::new("no_arc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    save_arc(black_box(lf_fm_index.clone()));
                }
            ));
        }
        // NO_Unarc
        {
            let lf_fm_index = build_unarc_64no(text_no.clone());

            // Save
            group.bench_with_input(
                BenchmarkId::new("no_unarc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    save_unarc(black_box(lf_fm_index.clone()));
                }
            ));
        }
        // AN_Arc
        {
            let lf_fm_index = build_arc_64an(text_an.clone());

            // Save
            group.bench_with_input(
                BenchmarkId::new("an_arc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    save_arc(black_box(lf_fm_index.clone()));
                }
            ));
        }
        // AN_Unarc
        {
            let lf_fm_index = build_unarc_64an(text_an.clone());

            // Save
            group.bench_with_input(
                BenchmarkId::new("an_unarc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    save_unarc(black_box(lf_fm_index.clone()));
                }
            ));
        }
    }
    group.finish();
}

// 3
// Load Bench
pub fn bench_load_arc_vs_unarc(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("load_arc_vs_unarc");

    group.plot_config(plot_config.clone());

    let text_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let text_an = rand_text_with_length(&UTF8_OF_AN, text_len);
        // NO_Arc
        {
            let lf_fm_index = build_arc_64no(text_no.clone());

            let buffer = save_arc(lf_fm_index);

            // Load
            group.bench_with_input(
                BenchmarkId::new("no_arc_chk", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_arc_checked(black_box(buffer.clone()));
                }
            ));

            group.bench_with_input(
                BenchmarkId::new("no_arc_unchk", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_arc_unchecked(black_box(buffer.clone()));
                }
            ));
        }
        // NO_Unarc
        {
            let lf_fm_index = build_unarc_64no(text_no.clone());

            let buffer = save_unarc(lf_fm_index);

            // Load
            group.bench_with_input(
                BenchmarkId::new("no_unarc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_unarc(black_box(buffer.clone()));
                }
            ));
        }
        // AN_Arc
        {
            let lf_fm_index = build_arc_64an(text_an.clone());

            let buffer = save_arc(lf_fm_index);
            // Load
            group.bench_with_input(
                BenchmarkId::new("an_arc_chk", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_arc_checked(black_box(buffer.clone()));
                }
            ));

            group.bench_with_input(
                BenchmarkId::new("an_arc_unchk", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_arc_unchecked(black_box(buffer.clone()));
                }
            ));
        }
        // AN_Unarc
        {
            let lf_fm_index = build_unarc_64an(text_an.clone());

            let buffer = save_unarc(lf_fm_index);
            // Load
            group.bench_with_input(
                BenchmarkId::new("an_unarc", text_len),
                &text_len,
                |b, i| b.iter(|| {
                    load_unarc(black_box(buffer.clone()));
                }
            ));
        }
    }
    group.finish();
}

// 4
// Locate Bench
pub fn bench_locate_arc_vs_unarc(c: &mut Criterion) {
    let mut group = c.benchmark_group("locate_arc_vs_unarc");

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config.clone());

    let text_len = 100_000_000;
    let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
    let text_an = rand_text_with_length(&UTF8_OF_AN, text_len);

    let lt_fm_index_no_arc = build_arc_64no(text_no.clone());
    let lt_fm_index_no_unarc = build_unarc_64no(text_no.clone());
    let lt_fm_index_an_arc = build_arc_64an(text_an.clone());
    let lt_fm_index_an_unarc = build_unarc_64an(text_an.clone());

    let pattern_len_list: Vec<usize> = (1..=6).map(|v| 10_usize.pow(v)).collect();

    for pattern_len in pattern_len_list {
        let pattern_no = &text_no[..pattern_len];
        let pattern_an = &text_an[..pattern_len];

        group.bench_with_input(
            BenchmarkId::new("no_arc", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                lt_fm_index_no_arc.locate(black_box(pattern_no));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("no_unarc", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                lt_fm_index_no_unarc.locate(black_box(pattern_no));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("an_arc", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                lt_fm_index_an_arc.locate(black_box(pattern_an));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("an_unarc", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                lt_fm_index_an_unarc.locate(black_box(pattern_an));
            }
        ));
    }
    group.finish();
}