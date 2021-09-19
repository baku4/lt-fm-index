use std::ops::Range;

use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::*;
use lt_fm_index::tests::random_text::*;

// Bench performance version of new vs deprecated

pub fn bench_generate_new_vs_dep(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("generate_new_vs_dep");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (2..=8).map(|v| 4_usize.pow(v)).collect();

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let text_nn = rand_text_with_length(&UTF8_OF_NN, text_len);

        group.bench_with_input(
            BenchmarkId::new("no_with_new", text_len),
            &text_len,
            |b, i| b.iter(|| {
                generate_new_no(black_box(text_no.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("no_with_old", text_len),
            &text_len,
            |b, i| b.iter(|| {
                generate_old_no(black_box(text_no.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("nn_with_new", text_len),
            &text_len,
            |b, i| b.iter(|| {
                generate_new_nn(black_box(text_nn.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("nn_with_old", text_len),
            &text_len,
            |b, i| b.iter(|| {
                generate_old_nn(black_box(text_nn.clone()));
            }
        ));
    }
    group.finish();
}

pub fn bench_locate_new_vs_dep(c: &mut Criterion) {
    let mut group = c.benchmark_group("locate_new_vs_dep");

    let text_len = 1000;
    let text = rand_text_with_length(&UTF8_OF_NO, text_len);

    let fm_index_new_no = generate_new_no(text.clone());
    let fm_index_new_nn = generate_new_nn(text.clone());
    let fm_index_old_no = generate_old_no(text.clone());
    let fm_index_old_nn = generate_old_nn(text.clone());

    let pattern_len_list: Vec<usize> = (2..=20).collect();

    for pattern_len in pattern_len_list {
        let pattern = &text[..pattern_len];

        group.bench_with_input(
            BenchmarkId::new("no_with_new", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                fm_index_new_no.locate(black_box(pattern));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("no_with_old", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                fm_index_old_no.locate_w_klt(black_box(pattern));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("nn_with_new", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                fm_index_new_nn.locate(black_box(pattern));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("nn_with_old", pattern_len),
            &pattern_len,
            |b, i| b.iter(|| {
                fm_index_old_nn.locate_w_klt(black_box(pattern));
            }
        ));
    }
    group.finish();
}

const KMER_SIZE: usize = 8;
const SA_SAMPLING_RATIO: u64 = 1;

#[inline]
fn generate_new_no(text: Text) -> Box<dyn FmIndex> {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(KMER_SIZE).unwrap()
        .change_suffix_array_sampling_ratio(SA_SAMPLING_RATIO).unwrap()
        .generate(text)
}

#[inline]
fn generate_new_nn(text: Text) -> Box<dyn FmIndex> {
    LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(KMER_SIZE).unwrap()
        .change_suffix_array_sampling_ratio(SA_SAMPLING_RATIO).unwrap()
        .generate(text)
}

#[inline]
fn generate_old_no(text: Text) -> deprecated::FmIndexDep {
    deprecated::FmIndexConfigDep::new()
        .set_kmer_lookup_table(KMER_SIZE)
        .set_suffix_array_sampling_ratio(SA_SAMPLING_RATIO)
        .generate_fmindex(text)
}

#[inline]
fn generate_old_nn(text: Text) -> deprecated::FmIndexDep {
    deprecated::FmIndexConfigDep::new()
        .set_kmer_lookup_table(KMER_SIZE)
        .set_suffix_array_sampling_ratio(SA_SAMPLING_RATIO)
        .contain_non_nucleotide()
        .generate_fmindex(text)
}
