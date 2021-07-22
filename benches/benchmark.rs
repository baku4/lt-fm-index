use std::fmt::format;
use std::iter;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use criterion::BenchmarkId;
use criterion::Throughput;

use lt_fm_index::{Config, FmIndex};

use fm_index::converter::RangeConverter;
use fm_index::suffix_array::SuffixOrderSampler;
use fm_index::{BackwardSearchIndex, FMIndex};
use fm_index::suffix_array::SuffixOrderSampledArray;


// crate lt-fm-index
fn index_generation_ltfmindex_without_klt(text: Vec<u8>) -> FmIndex {
    let config = Config::new()
        .set_suffix_array_sampling_ratio(2);
    FmIndex::new(&config, text)
}

fn index_generation_ltfmindex_with_klt(text: Vec<u8>, kmer_size: usize) -> FmIndex {
    let config = Config::new()
        .set_suffix_array_sampling_ratio(2)
        .set_kmer_lookup_table(kmer_size);
    FmIndex::new(&config, text)
}

fn ltfmindex_locate_without_klt(fm_index: &FmIndex, pattern: &[u8]) {
    let _ = fm_index.locate(pattern);
}

fn ltfmindex_locate_with_klt(fm_index: &FmIndex, pattern: &[u8]) {
    let _ = fm_index.locate_with_klt(pattern);
}

// crate fm-index
fn index_generation_cratefmindex(text: Vec<u8>) -> FMIndex<u8, RangeConverter<u8>, SuffixOrderSampledArray> {
    let converter = RangeConverter::new(b' ', b'~');
    let sampler = SuffixOrderSampler::new().level(2);
    let index = FMIndex::new(text, converter, sampler);
    index
}

fn cratefmindex_locate(fm_index: &FMIndex<u8, RangeConverter<u8>, SuffixOrderSampledArray>, pattern: &[u8]) {
    let search = fm_index.search_backward(pattern);
    let _ = search.locate();
}

// fn klt_generation(c: &mut Criterion) {
//     let mut group = c.benchmark_group("klt generation");
//     for kmer_size in [2_usize, 3, 4, 5, 6, 7, 8, 9, 10].iter() {
//         group.bench_with_input(
//             BenchmarkId::from_parameter(kmer_size),
//             kmer_size,
//             |b, &kmer_size| {
//             b.iter(|| lt_fm_generate_with_kmer(kmer_size));
//         });
//     }
//     group.finish();
// }

// fn no_klt_generation(c: &mut Criterion) {
//     let mut group = c.benchmark_group("klt generation");
//     group.bench_function(
//         "no klt", |b| {
//             b.iter(|| lt_fm_generate_without_klt());
//         }
//     );
//     group.finish();
// }

fn bench_search_with_klt(c: &mut Criterion) {
    // Preparing data
    let fm_index_8 = {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = Config::new()
            .set_kmer_lookup_table(8)
            .set_suffix_array_sampling_ratio(2);
        FmIndex::new(&config, text)
    };
    let pattern_list = vec![
        "G", "TA", "CCA", "TTCC", "GCACC", "AGCACGATCTG", "TTCCCCATTTTGCGTACCGGAAA"
    ];

    let mut group = c.benchmark_group("search with klt");
    for pattern in pattern_list{
        group.bench_with_input(
            format!("{} - klt", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| ltfmindex_locate_with_klt(&fm_index_8, pattern.as_bytes()));
        });
        group.bench_with_input(
            format!("{} - noklt", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| ltfmindex_locate_without_klt(&fm_index_8, pattern.as_bytes()));
        });
    }
    group.finish();
}

fn bench_index_generation_time(c: &mut Criterion) {
    // Preparing data
    // len: 1,230
    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();

    let mut group = c.benchmark_group("generation btw crates");
    for &text_len in [10_usize, 50, 100, 500, 1000].iter() {
        let sliced_text = text[..text_len].to_vec();
        group.bench_with_input(
            format!("{} - ltfmindex", text_len),
            &sliced_text,
            |b, text| {
            b.iter(|| index_generation_ltfmindex_without_klt(text.clone()));
        });
        group.bench_with_input(
            format!("{} - cratefmindex", text_len),
            &sliced_text,
            |b, text| {
            b.iter(|| index_generation_cratefmindex(text.clone()));
        });
    }
    group.finish();
}

fn bench_index_locate_time(c: &mut Criterion) {
    // Preparing data
    // len: 1,230
    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
    let lt_fm_index = index_generation_ltfmindex_without_klt(text.clone());
    let cratefm_index = index_generation_cratefmindex(text.clone());
    let pattern_list = vec![
        "G", "TA", "CCA", "TTCC", "GCACC", "AGCACGATCTG", "TTCCCCATTTTGCGTACCGGAAA"
    ];

    let mut group = c.benchmark_group("locate btw crates");
    for pattern in pattern_list {
        group.bench_with_input(
            format!("{} - ltfmindex", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| ltfmindex_locate_without_klt(&lt_fm_index, pattern.as_bytes()));
        });
        group.bench_with_input(
            format!("{} - cratefmindex", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| cratefmindex_locate(&cratefm_index, pattern.as_bytes()));
        });
    }
    group.finish();
}

fn bench_index_gen_locate_time(c: &mut Criterion) {
    // Preparing data
    // len: 1,230
    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
    let pattern_list = vec![
        "G", "TA", "CCA", "TTCC", "GCACC", "AGCACGATCTG", "TTCCCCATTTTGCGTACCGGAAA"
    ];

    let mut group = c.benchmark_group("gen&locate btw crates");
    for pattern in pattern_list {
        group.bench_with_input(
            format!("{} - ltfmindex", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| {
                let lt_fm_index = index_generation_ltfmindex_without_klt(text.clone());
                ltfmindex_locate_without_klt(&lt_fm_index, pattern.as_bytes());
            })
        });
        group.bench_with_input(
            format!("{} - cratefmindex", pattern.len()),
            pattern,
            |b, pattern| {
            b.iter(|| {
                let cratefm_index = index_generation_cratefmindex(text.clone());
                cratefmindex_locate(&cratefm_index, pattern.as_bytes());
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_index_generation_time, bench_index_locate_time, bench_index_gen_locate_time);
criterion_main!(benches);