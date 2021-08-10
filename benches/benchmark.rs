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
Compare to crate `fm-index`
*/
use fm_index::converter::RangeConverter;
use fm_index::suffix_array::{SuffixOrderSampler, SuffixOrderSampledArray};
use fm_index::{BackwardSearchIndex, FMIndex};

fn generate_index_of_crate_fm_index(ssr: usize, text: &Vec<u8>) -> FMIndex<u8, RangeConverter<u8>, SuffixOrderSampledArray> {
    let converter = RangeConverter::new(b' ', b'~');
    let sampler = SuffixOrderSampler::new().level(ssr);
    FMIndex::new(text.clone(), converter, sampler)
}
fn generate_index_of_lt_fm_index_on(ssr: u64, kmer: usize,text: Vec<u8>) -> FmIndex {
    let config = FmIndexConfig::new()
        .set_kmer_lookup_table(kmer)
        .set_suffix_array_sampling_ratio(ssr);
    config.generate_fmindex(text)
}
fn generate_index_of_lt_fm_index_nn(ssr: u64, kmer: usize,text: Vec<u8>) -> FmIndex {
    let config = FmIndexConfig::new()
        .set_kmer_lookup_table(kmer)
        .set_suffix_array_sampling_ratio(ssr)
        .contain_non_nucleotide();
    config.generate_fmindex(text)
}
fn text_1000_on() -> Vec<u8> {
    "CTCCGTGATTTACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATTGTTGCTGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGATTTACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGATTTACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACCCTCCGTACACCTGTTTCGTATCGGAACCGGTAAAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCG".as_bytes().to_vec()
}
fn pattern_100_on() -> Vec<u8> {
   b"CGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATTGTTGCTGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACC".to_vec()
}
fn bench_generate_by_crate(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 8;

    let text = text_1000_on();

    let mut group = c.benchmark_group("generate_index_by_crate");

    for i in [100_usize,250,500,750,1000].iter() {
        let text_sliced = text[..*i].to_vec();
        group.bench_with_input(
            BenchmarkId::new("Crate-fm-index", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_crate_fm_index(black_box(ssr), black_box(&text_sliced))
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-on", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_lt_fm_index_on(black_box(ssr as u64), black_box(kmer), black_box(text_sliced.clone()))
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-nn", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_lt_fm_index_nn(black_box(ssr as u64), black_box(kmer), black_box(text_sliced.clone()))
            }
        ));
    }
    group.finish();
}
fn bench_locate_by_crate(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 8;

    let text = text_1000_on();

    let crate_fm_index = generate_index_of_crate_fm_index(ssr, &text);
    let lt_fm_index_on = generate_index_of_lt_fm_index_on(ssr as u64, kmer, text.clone());
    let lt_fm_index_nn = generate_index_of_lt_fm_index_nn(ssr as u64, kmer, text.clone());

    let mut group = c.benchmark_group("locate_pattern_by_crate");

    let pattern = pattern_100_on();

    let pattern_len: Vec<usize> = (1..=20).into_iter().map(|x| x*5).collect();

    for i in &pattern_len {
        let pattern_sliced = pattern[..*i].to_vec();
        group.bench_with_input(
            BenchmarkId::new("Crate-fm-index", i),
            i, 
            |b, i| b.iter(|| {
                crate_fm_index.search_backward(&pattern_sliced).locate();
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-on", i),
            i, 
            |b, i| b.iter(|| {
                lt_fm_index_on.locate_w_klt(&pattern_sliced);
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-nn", i),
            i, 
            |b, i| b.iter(|| {
                lt_fm_index_nn.locate_w_klt(&pattern_sliced);
            }
        ));
    }
    group.finish();
}
fn bench_generate_and_locate_by_crate(c: &mut Criterion) {
    let ssr = 2;
    let kmer = 8;

    let text = text_1000_on();

    let mut group = c.benchmark_group("generate_and_locate_by_crate");

    let pattern = pattern_100_on();

    let pattern_len: Vec<usize> = (1..=20).into_iter().map(|x| x*5).collect();

    for i in &pattern_len {
        let pattern_sliced = pattern[..*i].to_vec();
        group.bench_with_input(
            BenchmarkId::new("Crate-fm-index", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_crate_fm_index(black_box(ssr), black_box(&text)).search_backward(black_box(&pattern_sliced)).locate();
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-on", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_lt_fm_index_on(black_box(ssr as u64), black_box(kmer), black_box(text.clone())).locate_w_klt(black_box(&pattern_sliced));
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-nn", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_lt_fm_index_nn(black_box(ssr as u64), black_box(kmer), black_box(text.clone())).locate_w_klt(black_box(&pattern_sliced));
            }
        ));
    }
    group.finish();
}
fn bench_no_klt_generate_and_locate_by_crate(c: &mut Criterion) {
    let ssr = 2;

    let text = text_1000_on();

    let mut group = c.benchmark_group("no_klt_generate_and_locate_by_crate");

    let pattern = pattern_100_on();

    let pattern_len: Vec<usize> = (1..=20).into_iter().map(|x| x*5).collect();

    for i in &pattern_len {
        let pattern_sliced = pattern[..*i].to_vec();
        group.bench_with_input(
            BenchmarkId::new("Crate-fm-index", i),
            i, 
            |b, i| b.iter(|| {
                generate_index_of_crate_fm_index(black_box(ssr), black_box(&text)).search_backward(black_box(&pattern_sliced)).locate();
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-on", i),
            i, 
            |b, i| b.iter(|| {
                FmIndexConfig::new()
                    .set_suffix_array_sampling_ratio(black_box(ssr as u64))
                    .generate_fmindex(black_box(text.clone()))
                    .locate_wo_klt(black_box(&pattern_sliced));
            }
        ));
        group.bench_with_input(
            BenchmarkId::new("Lt-fm-index-nn", i),
            i, 
            |b, i| b.iter(|| {
                FmIndexConfig::new()
                    .set_suffix_array_sampling_ratio(black_box(ssr as u64))
                    .contain_non_nucleotide()
                    .generate_fmindex(black_box(text.clone()))
                    .locate_wo_klt(black_box(&pattern_sliced));
            }
        ));
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

criterion_group!(benches, bench_no_klt_generate_and_locate_by_crate, bench_generate_and_locate_by_crate, bench_locate_by_crate, bench_generate_by_crate, bench_locate_by_pattern_length, bench_get_ca_klt);
criterion_main!(benches);