use criterion::{
    black_box, criterion_group, criterion_main, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};

use lt_fm_index::archived::composition::{
    SelfDescLtFmIndexPreBuild,
    TextType,
    BwtCompressionSize,
};
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

type SerializerType1 = AllocSerializer::<0>; // No scratch space
type SerializerType2 = AllocSerializer::<256>; // Small scratch space
type SerializerType3 = AllocSerializer::<512_000>; // Large scratch space


#[inline]
fn serialize_test_1(lt_fm_index_pre_build: SelfDescLtFmIndexPreBuild) {
    let mut serializer = SerializerType1::default();
    serializer.serialize_value(&lt_fm_index_pre_build).unwrap();
    let _ = serializer.into_serializer().into_inner();
}
#[inline]
fn serialize_test_2(lt_fm_index_pre_build: SelfDescLtFmIndexPreBuild) {
    let mut serializer = SerializerType2::default();
    serializer.serialize_value(&lt_fm_index_pre_build).unwrap();
    let _ = serializer.into_serializer().into_inner();
}
#[inline]
fn serialize_test_3(lt_fm_index_pre_build: SelfDescLtFmIndexPreBuild) {
    let mut serializer = SerializerType3::default();
    serializer.serialize_value(&lt_fm_index_pre_build).unwrap();
    let _ = serializer.into_serializer().into_inner();
}

pub fn bench_serialization_btw_serializer(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("serialization_btw_serializer");
    group.plot_config(plot_config);

    let text_len_list: Vec<usize> = (5..=8).map(|v| 10_usize.pow(v)).collect();

    let sa_sampling_ratio = 2;
    let kmer_size = 6;
    let text_type = TextType::NucleotideOnly;
    let bwt_compression_size = BwtCompressionSize::_64;

    for text_len in text_len_list {
        let text_no = rand_text_with_length(&UTF8_OF_NO, text_len);
        let lt_fm_index = SelfDescLtFmIndexPreBuild::new(
            text_no,
            sa_sampling_ratio,
            kmer_size,
            text_type.clone(),
            bwt_compression_size.clone(),
        );

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