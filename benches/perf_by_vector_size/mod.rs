use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};
use lt_fm_index::*;
use lt_fm_index::tests::random_text::{
    NO_STEMS, gen_rand_text,
    rand_pattern_of_length,
};

#[inline]
fn build_32(text: Vec<u8>, ss: u64, lk: u32) -> LtFmIndex<text_encoders::C3B32> {
    let te = text_encoders::C3B32::new(&[
        &[b'A', b'a'],
        &[b'C', b'c'],
        &[b'G', b'g'],
    ]);
    LtFmIndex::new(text, &te, ss, lk)
}
fn build_64(text: Vec<u8>, ss: u64, lk: u32) -> LtFmIndex<text_encoders::C3B64> {
    let te = text_encoders::C3B64::new(&[
        &[b'A', b'a'],
        &[b'C', b'c'],
        &[b'G', b'g'],
    ]);
    LtFmIndex::new(text, &te, ss, lk)
}
#[inline]
fn build_128(text: Vec<u8>, ss: u64, lk: u32) -> LtFmIndex<text_encoders::C3B128> {
    let te = text_encoders::C3B128::new(&[
        &[b'A', b'a'],
        &[b'C', b'c'],
        &[b'G', b'g'],
    ]);
    LtFmIndex::new(text, &te, ss, lk)
}
#[inline]
fn build_mul_texts_32(texts: Vec<Vec<u8>>, ss: u64, lk: u32) {
    for text in texts {
        _ = build_32(text, ss, lk);
    }
}
#[inline]
fn build_mul_texts_64(texts: Vec<Vec<u8>>, ss: u64, lk: u32) {
    for text in texts {
        _ = build_64(text, ss, lk);
    }
}
#[inline]
fn build_mul_texts_128(texts: Vec<Vec<u8>>, ss: u64, lk: u32) {
    for text in texts {
        _ = build_128(text, ss, lk);
    }
}
#[inline]
fn locate_mul_patterns_32(lfi: &LtFmIndex<text_encoders::C3B32>, patterns: &Vec<Vec<u8>>) {
    for pattern in patterns {
        _ = lfi.locate(pattern);
    }
}
#[inline]
fn locate_mul_patterns_64(lfi: &LtFmIndex<text_encoders::C3B64>, patterns: &Vec<Vec<u8>>) {
    for pattern in patterns {
        _ = lfi.locate(pattern);
    }
}
#[inline]
fn locate_mul_patterns_128(lfi: &LtFmIndex<text_encoders::C3B128>, patterns: &Vec<Vec<u8>>) {
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
            BenchmarkId::new("32", tl),
            &tl,
            |b, _i| b.iter(|| {
                build_mul_texts_32(black_box(texts.clone()), black_box(ss), black_box(lk));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("64", tl),
            &tl,
            |b, _i| b.iter(|| {
                build_mul_texts_64(black_box(texts.clone()), black_box(ss), black_box(lk));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("128", tl),
            &tl,
            |b, _i| b.iter(|| {
                build_mul_texts_128(black_box(texts.clone()), black_box(ss), black_box(lk));
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

    let lt_fm_index_32 = build_32(text.clone(), ss, lk);
    let lt_fm_index_64 = build_64(text.clone(), ss, lk);
    let lt_fm_index_128 = build_128(text.clone(), ss, lk);

    let pattern_lens: Vec<usize> = {
        (1..=4).map(| v | 10_i32.pow(v) as usize ).collect()
    };

    let n = 20;
    for pl in pattern_lens {
        let patterns: Vec<Vec<u8>> = (0..n).map(|_| rand_pattern_of_length(&text, pl)).collect();
        
        group.bench_with_input(
            BenchmarkId::new("32", pl),
            &pl,
            |b, _i| b.iter(|| {
                locate_mul_patterns_32(black_box(&lt_fm_index_32), black_box(&patterns));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("64", pl),
            &pl,
            |b, _i| b.iter(|| {
                locate_mul_patterns_64(black_box(&lt_fm_index_64), black_box(&patterns));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("128", pl),
            &pl,
            |b, _i| b.iter(|| {
                locate_mul_patterns_128(black_box(&lt_fm_index_128), black_box(&patterns));
            }
        ));
    }
    group.finish();
}
