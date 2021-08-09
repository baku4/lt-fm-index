use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lt_fm_index::*;

/*
Bench Use Enum vs Trait vs Struct
*/

/*
enum FmiEnums {
    OnlyNc(FmIndexOn),
    NonNc(FmIndexNn),
}

impl FmiEnums {
    fn new(config: &FmIndexConfig, text: Vec<u8>) -> Self {
        if config.only_nucleotide {
            Self::OnlyNc(FmIndexOn::new(config, text))
        } else {
            Self::NonNc(FmIndexNn::new(config, text))
        }
    }
}
impl FmIndexTrait for FmiEnums {
    fn count(&self, pattern: &[u8]) -> u64 {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.count(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.count(pattern)
            },
        }
    }
    fn locate_wo_klt(&self, pattern: &[u8]) -> Vec<u64> {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.locate_wo_klt(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.locate_wo_klt(pattern)
            },
        }
    }
    fn locate_w_klt(&self, pattern: &[u8]) -> Vec<u64> {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.locate_w_klt(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.locate_w_klt(pattern)
            },
        }
    }
}

// Genrating bench
fn fmi_generate_from_config(config: &FmIndexConfig, text: Vec<u8>) -> Box<dyn FmIndexTrait> {
    config.generate_fmindex_dep(text)
}
fn fmi_generate_from_enum(config: &FmIndexConfig, text: Vec<u8>) -> FmiEnums {
    FmiEnums::new(config, text)
}
fn fmi_generate_from_struct_on(config: &FmIndexConfig, text: Vec<u8>) -> FmIndexOn {
    FmIndexOn::new(config, text)
}
fn fmi_generate_from_struct_nn(config: &FmIndexConfig, text: Vec<u8>) -> FmIndexNn {
    FmIndexNn::new(config, text)
}
fn bench_generate_fmindex(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 4;

    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config_on = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer);
    let config_nn = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer)
        .contain_non_nucleotide();

    let mut group = c.benchmark_group("generate_fmi");
    
    // use new
    group.bench_function(
        "OnlyNc-Config",
        |b| b.iter(|| fmi_generate_from_config(black_box(&config_on), black_box(text.clone())))
    );
    group.bench_function(
        "OnlyNc-Struct",
        |b| b.iter(|| fmi_generate_from_struct_on(black_box(&config_on), black_box(text.clone())))
    );
    group.bench_function(
        "OnlyNc-Enum",
        |b| b.iter(|| fmi_generate_from_enum(black_box(&config_on), black_box(text.clone())))
    );
    group.bench_function(
        "NonNc-Config",
        |b| b.iter(|| fmi_generate_from_config(black_box(&config_nn), black_box(text.clone())))
    );
    group.bench_function(
        "NonNc-Struct",
        |b| b.iter(|| fmi_generate_from_struct_nn(black_box(&config_nn), black_box(text.clone())))
    );
    group.bench_function(
        "NonNc-Enum",
        |b| b.iter(|| fmi_generate_from_enum(black_box(&config_nn), black_box(text.clone())))
    );
    
    group.finish();
}

// Locate bench
fn bench_locate(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 4;

    let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec(); // length 500
    let config_on = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer);
    let config_nn = FmIndexConfig::new()
        .set_suffix_array_sampling_ratio(ssr)
        .set_kmer_lookup_table(kmer)
        .contain_non_nucleotide();
    
    let fmindex_on_config = fmi_generate_from_config(&config_on, text.clone());
    let fmindex_nn_config = fmi_generate_from_config(&config_nn, text.clone());
    let fmindex_on_enum = fmi_generate_from_enum(&config_on, text.clone());
    let fmindex_nn_enum = fmi_generate_from_enum(&config_nn, text.clone());
    let fmindex_on_struct = fmi_generate_from_struct_on(&config_on, text.clone());
    let fmindex_nn_struct = fmi_generate_from_struct_nn(&config_nn, text.clone());

    let mut group = c.benchmark_group("locate_fmi");

    let pattern = b"TGTTTCGTATCGGAACCGGTAAGTGAAATT".to_vec();

    group.bench_function(
        "OnlyNc-Config",
        |b| b.iter(|| fmindex_on_config.locate_w_klt(black_box(&pattern)))
    );
    group.bench_function(
        "OnlyNc-Struct",
        |b| b.iter(|| fmindex_on_struct.locate_w_klt(black_box(&pattern)))
    );
    group.bench_function(
        "OnlyNc-Enum",
        |b| b.iter(|| fmindex_on_enum.locate_w_klt(black_box(&pattern)))
    );
    group.bench_function(
        "NonNc-Config",
        |b| b.iter(|| fmindex_nn_config.locate_w_klt(black_box(&pattern)))
    );
    group.bench_function(
        "NonNc-Struct",
        |b| b.iter(|| fmindex_nn_struct.locate_w_klt(black_box(&pattern)))
    );
    group.bench_function(
        "NonNc-Enum",
        |b| b.iter(|| fmindex_nn_enum.locate_w_klt(black_box(&pattern)))
    );

    group.finish();
}

*/

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

criterion_group!(benches, bench_get_ca_klt);
criterion_main!(benches);