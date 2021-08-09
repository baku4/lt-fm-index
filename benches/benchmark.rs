use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lt_fm_index::*;

/*
Bench generating Fmindex using Box<dyn trait> or directly
*/
fn get_fmindex_direct_on(config: &FmIndexConfig, text: Vec<u8>) {
    let _ = fmindex_on::FmIndexOn::new(config, text);
}
fn get_fmindex_use_box_on(config: &FmIndexConfig, text: Vec<u8>) {
    let _ = config.generate_fmindex(text);
}
fn get_fmindex_direct_nn(config: &FmIndexConfig, text: Vec<u8>) {
    let _ = fmindex_nn::FmIndexNn::new(config, text);
}
fn get_fmindex_use_box_nn(config: &FmIndexConfig, text: Vec<u8>) {
    let _ = config.generate_fmindex(text);
}
fn bench_get_fmindex_use_box_or_not(c: &mut Criterion) {
    let ssr = 4;
    let kmer = 8;

    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config_on = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer);
    let config_nn = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer)
        .contain_non_nucleotide();

    let mut group = c.benchmark_group("generate_fmi_box");
    
    // use new
    group.bench_function(
        "OnlyNc-Dir",
        |b| b.iter(|| get_fmindex_direct_on(black_box(&config_on), black_box(text.clone())))
    );
    group.bench_function(
        "OnlyNc-Box",
        |b| b.iter(|| get_fmindex_use_box_on(black_box(&config_on), black_box(text.clone())))
    );
    group.bench_function(
        "NonNc-Dir",
        |b| b.iter(|| get_fmindex_direct_nn(black_box(&config_nn), black_box(text.clone())))
    );
    group.bench_function(
        "NonNc-Box",
        |b| b.iter(|| get_fmindex_use_box_nn(black_box(&config_nn), black_box(text.clone())))
    );
    
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

criterion_group!(benches, bench_get_fmindex_use_box_or_not, bench_get_ca_klt);
criterion_main!(benches);