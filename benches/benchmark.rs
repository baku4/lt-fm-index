use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lt_fm_index::*;


/*
Bench generating CA and KLT
*/

fn get_ca_klt_new(config: &Config, mut text: Vec<u8>) {
    let _ = fmindex_nn::FmIndexNn::get_ca_and_klt(config, &mut text);
}
fn get_ca_klt_old(config: &Config, mut text: Vec<u8>) {
    let _ = fmindex_on::FmIndexOn::get_ca_and_klt(config, &mut text);
}
fn bench_get_ca_klt(c: &mut Criterion) {
    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config = Config::new()
    .set_suffix_array_sampling_ratio(4)
    .set_kmer_lookup_table(8);

    let mut group = c.benchmark_group("get_ca_klt");
    
    // use new
    group.bench_function(
        "New",
        |b| b.iter(|| get_ca_klt_new(black_box(&config), black_box(text.clone())))
    );
    group.bench_function(
        "Old",
        |b| b.iter(|| get_ca_klt_old(black_box(&config), black_box(text.clone())))
    );
    group.finish();
}

criterion_group!(benches, bench_get_ca_klt);
criterion_main!(benches);