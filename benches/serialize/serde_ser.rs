use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::*;
use lt_fm_index::use_case::*;
use lt_fm_index::tests::random_text::*;

fn serialize_test_1(lt_fm_index: LtFmIndexAll) {
    let bytes = bincode::serialize(&lt_fm_index).unwrap();
}

pub fn bench_serialize_btw_bincode_serializers(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("serialize_with_bincode_ser");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (4..=7).map(|v| 10_usize.pow(v)).collect();

    let kmer_size = 4;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = LtFmIndexConfig::for_nucleotide().change_kmer_size(kmer_size).unwrap().generate(text_no).unwrap();

        group.bench_with_input(
            BenchmarkId::new("bincode", text_len),
            &text_len,
            |b, _| b.iter(|| {
                serialize_test_1(black_box(lt_fm_index.clone()));
            }
        ));
    }

    group.finish();
}

fn deserialize_test_1(bytes: &Vec<u8>) {
    let decoded: LtFmIndexAll = bincode::deserialize(bytes).unwrap();
}

pub fn bench_deserialize_btw_bincode_deserializers(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("deserialize_with_bincode_ser");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (4..=7).map(|v| 10_usize.pow(v)).collect();

    let kmer_size = 4;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = LtFmIndexConfig::for_nucleotide().change_kmer_size(kmer_size).unwrap().generate(text_no).unwrap();
        let bytes = bincode::serialize(&lt_fm_index).unwrap();

        group.bench_with_input(
            BenchmarkId::new("bincode", text_len),
            &text_len,
            |b, _| b.iter(|| {
                deserialize_test_1(black_box(&bytes));
            }
        ));
    }

    group.finish();
}