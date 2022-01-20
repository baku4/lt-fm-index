use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::*;
use lt_fm_index::tests::random_text::*;

const KMER_SIZE_NO: usize = 6;
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

// 1
// Save Bench
#[inline]
fn save_with_taking(lt_fm_index: LtFmIndex) -> Vec<u8> {
    lt_fm_index.take_inner_bytes()
}
#[inline]
fn save_with_writing(lt_fm_index: LtFmIndex) -> Vec<u8> {
    let mut buffer = Vec::new();
    lt_fm_index.save_to(&mut buffer);
    buffer
}

pub fn bench_save_taking_vs_writing(c: &mut Criterion) {
    let mut group = c.benchmark_group("save_taking_vs_writing");

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config.clone());

    let text_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = build_arc_64no(text_no);

        group.bench_with_input(
            BenchmarkId::new("taking", text_len),
            &text_len,
            |b, i| b.iter(|| {
                save_with_taking(black_box(lt_fm_index.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("writing", text_len),
            &text_len,
            |b, i| b.iter(|| {
                save_with_writing(black_box(lt_fm_index.clone()));
            }
        ));
    }
    group.finish();
}

// 2
// Load Bench
fn load_with_casting_chk(bytes: Vec<u8>) -> LtFmIndex {
    LtFmIndex::new_from_bytes_checked(bytes).unwrap()
}
fn load_with_casting_unchk(bytes: Vec<u8>) -> LtFmIndex {
    LtFmIndex::new_from_bytes_unchecked(bytes)
}
fn load_with_including(bytes: Vec<u8>) -> LtFmIndex {
    LtFmIndex::load_from(&bytes[..]).unwrap()
}

pub fn bench_load_casting_vs_including(c: &mut Criterion) {
    let mut group = c.benchmark_group("load_casting_vs_including");

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config.clone());

    let text_len_list: Vec<usize> = (4..=8).map(|v| 10_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = build_arc_64no(text_no);
        let bytes = lt_fm_index.take_inner_bytes();

        group.bench_with_input(
            BenchmarkId::new("casting_chk", text_len),
            &text_len,
            |b, i| b.iter(|| {
                load_with_casting_chk(black_box(bytes.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("casting_unchk", text_len),
            &text_len,
            |b, i| b.iter(|| {
                load_with_casting_unchk(black_box(bytes.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("including", text_len),
            &text_len,
            |b, i| b.iter(|| {
                load_with_including(black_box(bytes.clone()));
            }
        ));
    }
    group.finish();
}