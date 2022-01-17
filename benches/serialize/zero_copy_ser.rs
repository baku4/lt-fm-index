use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::*;
use lt_fm_index::use_case::*;
use lt_fm_index::tests::random_text::*;

use rkyv::{
    AlignedVec,
    Infallible,
    Deserialize,
    ser::{
        Serializer,
        serializers::{
            CompositeSerializer,
            AlignedSerializer,
            BufferScratch,
            SharedSerializeMap,
            AllocSerializer,
        }
    }
};

type SerializerType1 = AllocSerializer::<0>;
type SerializerType2 = AllocSerializer::<256>;
type SerializerType3 = AllocSerializer::<512_000>;

#[inline]
fn serialize_test_1(lt_fm_index: LtFmIndexAll) {
    let mut serializer = SerializerType1::default();
    serializer.serialize_value(&lt_fm_index).unwrap();
    let _ = serializer.into_serializer().into_inner();
}
#[inline]
fn serialize_test_2(lt_fm_index: LtFmIndexAll) {
    let mut serializer = SerializerType2::default();
    serializer.serialize_value(&lt_fm_index).unwrap();
    let _ = serializer.into_serializer().into_inner();
}
#[inline]
fn serialize_test_3(lt_fm_index: LtFmIndexAll) {
    let mut serializer = SerializerType3::default();
    serializer.serialize_value(&lt_fm_index).unwrap();
    let _ = serializer.into_serializer().into_inner();
}

pub fn bench_serialize_btw_zero_copy_serializers(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("serialize_with_zero_copy_ser");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (4..=7).map(|v| 10_usize.pow(v)).collect();

    let kmer_size = 4;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = LtFmIndexConfig::for_nucleotide().change_kmer_size(kmer_size).unwrap().generate(text_no).unwrap();

        group.bench_with_input(
            BenchmarkId::new("alloc_0", text_len),
            &text_len,
            |b, _| b.iter(|| {
                serialize_test_1(black_box(lt_fm_index.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("alloc_256", text_len),
            &text_len,
            |b, _| b.iter(|| {
                serialize_test_2(black_box(lt_fm_index.clone()));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("alloc_512000", text_len),
            &text_len,
            |b, _| b.iter(|| {
                serialize_test_3(black_box(lt_fm_index.clone()));
            }
        ));
    }

    group.finish();
}


#[inline]
fn deserialize_test_1(bytes: &[u8]) {
    let archived = unsafe{ rkyv::archived_root::<LtFmIndexAll>(&bytes) };
    let lt_fm_index: LtFmIndexAll = archived.deserialize(&mut rkyv::Infallible).unwrap();
}
#[inline]
fn deserialize_test_2(bytes: &[u8], pos: usize) {
    let archived = unsafe{ rkyv::archived_value::<LtFmIndexAll>(&bytes, pos) };
    let lt_fm_index: LtFmIndexAll = archived.deserialize(&mut rkyv::Infallible).unwrap();
}
#[inline]
fn deserialize_test_3(bytes: &[u8]) {
    let archived = rkyv::check_archived_root::<LtFmIndexAll>(&bytes).unwrap();
    let lt_fm_index: LtFmIndexAll = archived.deserialize(&mut rkyv::Infallible).unwrap();
}
#[inline]
fn deserialize_test_4(bytes: &[u8], pos: usize) {
    let archived = rkyv::check_archived_value::<LtFmIndexAll>(&bytes, pos).unwrap();
    let lt_fm_index: LtFmIndexAll = archived.deserialize(&mut rkyv::Infallible).unwrap();
}

pub fn bench_deserialize_btw_zero_copy_deserializers(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("deserialize_with_zero_copy_ser");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (4..=7).map(|v| 10_usize.pow(v)).collect();

    let kmer_size = 4;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = LtFmIndexConfig::for_nucleotide().change_kmer_size(kmer_size).unwrap().generate(text_no).unwrap();

        let mut serializer = SerializerType1::default();
        let pos = serializer.serialize_value(&lt_fm_index).unwrap();
        let bytes = serializer.into_serializer().into_inner();

        group.bench_with_input(
            BenchmarkId::new("root", text_len),
            &text_len,
            |b, _| b.iter(|| {
                deserialize_test_1(black_box(&bytes));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("value", text_len),
            &text_len,
            |b, _| b.iter(|| {
                deserialize_test_2(black_box(&bytes), pos);
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("check_root", text_len),
            &text_len,
            |b, _| b.iter(|| {
                deserialize_test_3(black_box(&bytes));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("check_value", text_len),
            &text_len,
            |b, _| b.iter(|| {
                deserialize_test_4(black_box(&bytes), pos);
            }
        ));
    }

    group.finish();
}

#[inline]
fn get_archived_test_1(bytes: &[u8]) {
    let archived = unsafe{ rkyv::archived_root::<LtFmIndexAll>(&bytes) };
}
#[inline]
fn get_archived_test_2(bytes: &[u8], pos: usize) {
    let archived = unsafe{ rkyv::archived_value::<LtFmIndexAll>(&bytes, pos) };
}
#[inline]
fn get_archived_test_3(bytes: &[u8]) {
    let archived = rkyv::check_archived_root::<LtFmIndexAll>(&bytes).unwrap();
}
#[inline]
fn get_archived_test_4(bytes: &[u8], pos: usize) {
    let archived = rkyv::check_archived_value::<LtFmIndexAll>(&bytes, pos).unwrap();
}

pub fn bench_get_archived(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("get_archived");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (4..=7).map(|v| 10_usize.pow(v)).collect();

    let kmer_size = 4;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = LtFmIndexConfig::for_nucleotide().change_kmer_size(kmer_size).unwrap().generate(text_no).unwrap();

        let mut serializer = SerializerType1::default();
        let pos = serializer.serialize_value(&lt_fm_index).unwrap();
        let bytes = serializer.into_serializer().into_inner();

        group.bench_with_input(
            BenchmarkId::new("root", text_len),
            &text_len,
            |b, _| b.iter(|| {
                get_archived_test_1(black_box(&bytes));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("value", text_len),
            &text_len,
            |b, _| b.iter(|| {
                get_archived_test_2(black_box(&bytes), pos);
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("check_root", text_len),
            &text_len,
            |b, _| b.iter(|| {
                get_archived_test_3(black_box(&bytes));
            }
        ));

        group.bench_with_input(
            BenchmarkId::new("check_value", text_len),
            &text_len,
            |b, _| b.iter(|| {
                get_archived_test_4(black_box(&bytes), pos);
            }
        ));
    }

    group.finish();
}
