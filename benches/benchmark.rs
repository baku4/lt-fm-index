use std::fmt::format;
use std::iter;

use criterion::{criterion_group, criterion_main, Criterion};
use criterion::BenchmarkId;
use criterion::{PlotConfiguration, AxisScale};

use lt_fm_index::{Config, FmIndex};

use seq::*;

// LT Fm-index
fn generate_without_klt(text: Vec<u8>) -> FmIndex {
    let config = Config::new()
        .set_suffix_array_sampling_ratio(2);
    FmIndex::new(&config, text)
}

fn generate_with_klt(text: Vec<u8>, kmer_size: usize) -> FmIndex {
    let config = Config::new()
        .set_suffix_array_sampling_ratio(2)
        .set_kmer_lookup_table(kmer_size);
    FmIndex::new(&config, text)
}

fn locate_without_klt(fm_index: &FmIndex, pattern: &[u8]) {
    let _ = fm_index.locate(pattern);
}

fn locate_with_klt(fm_index: &FmIndex, pattern: &[u8]) {
    let _ = fm_index.locate_with_klt(pattern);
}


// Benches
fn bench_klt_generation(c: &mut Criterion) {
    let text = text_1744();

    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    
    let mut group = c.benchmark_group("Generate KLT");
    group.bench_function(
        BenchmarkId::from_parameter(1), |b| {
            b.iter(|| generate_without_klt(text.clone()));
        }
    );
    for kmer_size in [02_usize, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(kmer_size),
            kmer_size,
            |b, &kmer_size| {
            b.iter(|| generate_with_klt(text.clone(), kmer_size));
        });
    }

    group.plot_config(plot_config);
    group.finish();
}

fn bench_locate_with_klt(c: &mut Criterion) {
    let text = text_1744();
    let pattern_list = ["A", "AC", "GAT", "TGAA", "ACGAT", "TTGCAGGGTG", "TGTGGAGATTGTAAC", "GAGCTACGTGAGGCATTGGA", "GTGCCTTATACGATGAGATTAAAAG"];
    let fm_index_8 = generate_with_klt(text.clone(), 8);
    
    let mut group = c.benchmark_group("Locate w/wo KLT");

    for &pattern in pattern_list.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:02}-K", pattern.len())),
            pattern,
            |b, pattern| {
            b.iter(|| locate_with_klt(&fm_index_8, pattern.as_bytes()));
        });
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:02}-N", pattern.len())),
            pattern,
            |b, pattern| {
            b.iter(|| locate_without_klt(&fm_index_8, pattern.as_bytes()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_klt_generation, bench_locate_with_klt);
criterion_main!(benches);

mod seq {
    pub fn text_1744() -> Vec<u8> {
        // ERR209055 - k59_14684 len=1744
        b"TCAGAGTTTTGGGATTTCTGGAAGCTACGCGAGAAATGATTATAAGGCCAGCAGTGATATCGACTTTTGCATCATAGTTCCGGAAAAACCGGAAAGATGGATGATGGGAGCCTTGCGGGAAGAACTCGAGATGTTGCATGCGGATGTGGTTTTTGTAACACCGCAGTATTTCGAACATGATAATTCAAAGTTTACACAACAGTTAAGAAGAGATTATAAGGAGTTGAAAATATGATAAAGGTAAATTCTTATTATTCTATTGCGTGCAATGAATTCTGGTATTTAAGGGATGCAGTGTCACCTGTATACTGCAATCCGGCTGCAGCTTCGGCACAGCAAATTGCAGAAAAAATGTTGAACTCTGTGGCAGAACTGGTTTGTACTGGAATTGAAAAACTAATGACATCCCATAATTTGCGTGCCTTATACGATGAGATTAAAAGGGTGGATACTTCCTTGCAATTAGAGAGGAAGGATCTGGCATTACTCAAAGACTATTATTATGATGCAAGATACCCGGGGGATAATTTTGTTATTGTGACAGTGGATGAGCTACGTGAGGCATTGGAGATCATGCTTGACGTTGTAGAAGCTGTGAATAGCTGGAGAACGTCTCATGAATTGGAGATATTAATTGCTGATCCGAGAGGGGAGTTTCAATCAGCGATGAGCAGATTGAAGCAGGAAGGGTAAACTGCCAAAATGAGCGGTTACAGTTTGCAGGGTGACTTGTGAACTGTAACGGAAAGAGTACGATTGAAAAATGATTTCCAAAAGATAAAAGACCATAAATAAGGGAGAATACCCATGGACGATACAAAAACAGAAATCATAAAACAAATATGCACAGGAGACTGGAAGGCAATCCCGGTTACAATGGGATTAAAATTTTTGAATGAACACCAGTTTACCCAAATCTTTGTGGATGCCGGAAAACAAATCAGGGAGTTTCGGCGGTATGGAACCGAAGAAACGGAACTTCGCAATGTTATATTTTGTGAAGAAAATATGAAATTACTGGCAAAACACATGCGCCCGGTAGATGATTACACGTGGTTGGAGAGACTCGTGATCTGTGTCGACAACTTAGTCAATGGGAGTGCTATGTCTGAGGATAATAAAAAAACGAGTAAACAACGTTTTGTGGAGATTGTAACAGACAAATGCCGTAAAATTCTGCCGGAAATTTATGGAAGATCACGGCTTGAAGAAATTGGTCAGGGAATGGAACAAATGAACAGCCGAATGGATCGGTTAGAGCGTGCGACACAAAATATCATTGATATGGGGAGCCGACAGAACCAGGAAGAACGTCTGCAGCAGAGGATGGAGACAGAAAAGAGACCGGAGACAGAGAAAAACATGCAGCAGAATGGTCCAACGGATATAGGGCAAATAGCCAAATGGGATTTATCCGATCAACAGGTGGAAGGACTGTTTGGATCAAAAGAAAACCGGTACAAAGATATCAGTCAGCTTACTGAAGCATGGAAGAAAGAACGTAATGTTTATCCGGGGTGGTACATTCTTCCCTACAACATATGCGCAGAACTAAACAGCAAAACAAGAGAAGAAGGACTGTTGCAAAGCCATACATTTGTTGATTTAAATCGGATGTTTCTGTTTGCATATGAACTTGCGTGGCGCTACGAAAAATGTATGCATTTGTATTCCGATTATGAGATCCATCACTTGTCCATAATCTGGGACAATTATTATGAAAAGGAAATCAAAAGCTGGAGTC".to_vec()
    }
}