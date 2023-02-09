use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};
use lt_fm_index::*;
use lt_fm_index::core::FmIndexInterface;
use lt_fm_index::tests::random_text::{
    NO_STEMS, gen_rand_text,
    rand_pattern_of_length,
};

fn build_raw(text: Vec<u8>, ss: u64, lk: u32) -> LtFmIndex<text_encoders::C3B64> {
    let te = text_encoders::C3B64::new(&[
        &[b'A'],
        &[b'C'],
        &[b'G'],
    ]);
    LtFmIndex::new(text, &te, ss, lk)
}
fn build_boxed(text: Vec<u8>, ss: u64, lk: u32) -> Box<dyn FmIndexInterface> {
    LtFmIndexBuilder::new()
        .add_chr(&[b'A'])
        .add_chr(&[b'C'])
        .add_chr(&[b'G'])
        .set_lookup_table_kmer_size(lk).unwrap()
        .set_suffix_array_sampling_ratio(ss).unwrap()
        .bwt_block_size_is_64()
        .build(text).unwrap()
}
fn build_mul_texts_raw(texts: Vec<Vec<u8>>, ss: u64, lk: u32) {
    for text in texts {
        _ = build_raw(text, ss, lk);
    }
}
fn build_mul_texts_boxed(texts: Vec<Vec<u8>>, ss: u64, lk: u32) {
    for text in texts {
        _ = build_boxed(text, ss, lk);
    }
}
fn locate_mul_patterns_raw(lfi: &LtFmIndex<text_encoders::C3B64>, patterns: &Vec<Vec<u8>>) {
    for pattern in patterns {
        _ = lfi.locate(pattern);
    }
}
fn locate_mul_patterns_boxed(lfi: &Box<dyn FmIndexInterface>, patterns: &Vec<Vec<u8>>) {
    for pattern in patterns {
        _ = lfi.locate(pattern);
    }
}

pub fn build_no_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("build");

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config.clone());

    let n = 20;
    let ss = 2;
    let lk = 4;

    let text_lens: Vec<usize> = {
        (1..=6).map(| v | 10_i32.pow(v) as usize ).collect()
    };

    for tl in text_lens {
        let texts: Vec<Vec<u8>> = (0..n).map(|_| gen_rand_text(&NO_STEMS, tl..tl+1)).collect();
        
        group.bench_with_input(
            BenchmarkId::new("raw", tl),
            &tl,
            |b, _i| b.iter(|| {
                build_mul_texts_raw(black_box(texts.clone()), black_box(ss), black_box(lk));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("boxed", tl),
            &tl,
            |b, _i| b.iter(|| {
                build_mul_texts_boxed(black_box(texts.clone()), black_box(ss), black_box(lk));
            }
        ));
    }
    group.finish();
}

pub fn locate_no_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("locate");
    
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config.clone());

    
    let ss = 2;
    let lk = 4;
    let text_len = 10_i32.pow(6) as usize;
    let text = gen_rand_text(&NO_STEMS, text_len..text_len+1);

    let lt_fm_index_old = build_raw(text.clone(), ss, lk);
    let lt_fm_index_new = build_boxed(text.clone(), ss, lk);

    let pattern_lens: Vec<usize> = {
        (1..=4).map(| v | 10_i32.pow(v) as usize ).collect()
    };

    let n = 20;
    for pl in pattern_lens {
        let patterns: Vec<Vec<u8>> = (0..n).map(|_| rand_pattern_of_length(&text, pl)).collect();
        
        group.bench_with_input(
            BenchmarkId::new("raw", pl),
            &pl,
            |b, _i| b.iter(|| {
                locate_mul_patterns_raw(black_box(&lt_fm_index_old), black_box(&patterns));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("boxed", pl),
            &pl,
            |b, _i| b.iter(|| {
                locate_mul_patterns_boxed(black_box(&lt_fm_index_new), black_box(&patterns));
            }
        ));
    }
    group.finish();
}
