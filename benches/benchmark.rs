use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId
};
use lt_fm_index::*;

/*
Bench Use Enum vs Trait vs Struct
*/

fn bench_locate_by_pattern_length(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 8;

    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer);
    let pattern = b"TATGGTTCAATGGCACCGGA".to_vec();
    let fm_index = config.generate_fmindex(text);
    
    // USE KLT
    let mut group = c.benchmark_group("locate_by_length_w_klt");
    for pattern_len in 1..=20 {
        let pattern = pattern[..pattern_len].to_vec();
        
        group.bench_function(
            BenchmarkId::from_parameter(pattern_len),
            |b| {
                b.iter(|| fm_index.locate_w_klt(black_box(&pattern)));
            }
        );
    }
    group.finish();

    // NOT KLT
    let mut group = c.benchmark_group("locate_by_length_wo_klt");
    for pattern_len in 1..=20 {
        let pattern = pattern[..pattern_len].to_vec();
        
        group.bench_function(
            BenchmarkId::from_parameter(pattern_len),
            |b| {
                b.iter(|| fm_index.locate_wo_klt(black_box(&pattern)));
            }
        );
    }
    group.finish();
}

/*
Bench generating CA and KLT
*/
fn get_ca_klt_nn(config: &FmIndexConfig, mut text: Vec<u8>) {
    let _ = fmindex_nn::FmIndexNn::get_ca_and_klt(config, &mut text);
}
fn get_ca_klt_on(config: &FmIndexConfig, mut text: Vec<u8>) {
    let _ = fmindex_on::FmIndexOn::get_ca_and_klt(config, &mut text);
}
fn bench_get_ca_klt(c: &mut Criterion) {
    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config = FmIndexConfig::new()
    .set_suffix_array_sampling_ratio(4)
    .set_kmer_lookup_table(8);

    let mut group = c.benchmark_group("get_ca_klt");
    
    // use new
    group.bench_function(
        "NonNc",
        |b| b.iter(|| get_ca_klt_nn(black_box(&config), black_box(text.clone())))
    );
    group.bench_function(
        "OnlyNc",
        |b| b.iter(|| get_ca_klt_on(black_box(&config), black_box(text.clone())))
    );
    group.finish();
}

criterion_group!(benches, bench_locate_by_pattern_length, bench_get_ca_klt);
criterion_main!(benches);