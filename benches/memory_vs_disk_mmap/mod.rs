use criterion::{
    black_box, Criterion, BenchmarkId,
    PlotConfiguration, AxisScale,
};
use lt_fm_index::{
    Position,
    new_algorithm::{
        FmIndex, FmIndexBuilder, Block,
        blocks::{Block2, Block3, Block4, Block5, Block6},
    },
};
use super::random_data::{
    gen_rand_chr_list,
    gen_rand_text,
    gen_rand_pattern,
};
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;
use memmap2::Mmap;

fn safe_filename(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn bench_memory_vs_disk_mmap_locate<P: Position, B: Block + 'static>(c: &mut Criterion) {
    // BIG_TEXT 환경변수 확인
    let big_text = std::env::var("BIG_TEXT").is_ok();
    
    //
    let p_name = std::any::type_name::<P>();
    let b_name = std::any::type_name::<B>().split("::").last().unwrap();

    let mut group = c.benchmark_group(format!("memory_vs_disk_mmap_locate_{}_{}", p_name, b_name));
    
    // 테스트 데이터 준비 - Block 타입에 맞게 문자 수 조정
    let max_chr_count = B::MAX_CHR as usize;
    let chr_list = gen_rand_chr_list(max_chr_count);
    
    // text 길이 설정 (BIG_TEXT 환경변수에 따라)
    let text_len = if big_text { 1_000_000_000 } else { 100_000 };
    let mut text = gen_rand_text(&chr_list, text_len, text_len);
    
    // 패턴 길이를 10부터 100까지 설정
    let pattern_lengths = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let n_patterns = 10; // 각 패턴 길이당 10개씩
    
    // 각 패턴 길이에 대해 패턴을 생성하고 text에 강제 삽입
    let mut patterns_by_length: Vec<Vec<Vec<u8>>> = Vec::new();
    
    for &pattern_len in &pattern_lengths {
        let mut patterns = Vec::new();
        for i in 0..n_patterns {
            // 패턴 생성
            let pattern = gen_rand_pattern(&text, pattern_len, pattern_len);
            
            // text에 강제 삽입 (5곳에 삽입)
            for j in 0..5 {
                let insert_pos = (i * (text_len / n_patterns) + j * (text_len / 5)) % (text_len - pattern_len);
                text[insert_pos..insert_pos + pattern_len].copy_from_slice(&pattern);
            }
            
            patterns.push(pattern);
        }
        patterns_by_length.push(patterns);
    }
    
    // FM-index 빌드
    let characters_by_index = chr_list.chunks(1).map(|c| c).collect::<Vec<_>>();
    let builder = FmIndexBuilder::<P, B>::init(
        text.len(),
        &characters_by_index,
        2, // sasr
        3, // ltks
    ).unwrap();
    
    let blob_size = builder.blob_aligned_size();
    let mut blob: Vec<u8> = vec![0; blob_size];
    builder.build(text, &mut blob).unwrap();
    
    let fm_index_memory = FmIndex::<P, B>::load(&blob).unwrap();
    
    // 디스크에 저장할 임시 파일 경로 (target 디렉토리 안에 생성)
    let temp_file = format!(
        "target/temp_fm_index_{}_{}_{}.bin",
        safe_filename(p_name),
        safe_filename(b_name),
        if big_text { "big" } else { "small" }
    );
    
    // target 디렉토리가 없으면 생성
    if !Path::new("target").exists() {
        fs::create_dir("target").unwrap();
    }
    
    // 디스크에 저장
    fs::write(&temp_file, &blob).unwrap();
    
    // mmap으로 파일을 메모리에 매핑
    let file = fs::File::open(&temp_file).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let fm_index_mmap = FmIndex::<P, B>::load(&mmap).unwrap();
    
    // 각 패턴 길이에 대해 벤치마크 실행
    for (pattern_len, patterns) in pattern_lengths.iter().zip(patterns_by_length.iter()) {
        // 메모리 locate 벤치마크
        group.bench_with_input(
            BenchmarkId::new("memory_locate", pattern_len),
            pattern_len,
            |b, _| {
                b.iter(|| {
                    for pattern in patterns {
                        black_box(fm_index_memory.locate_pattern(pattern));
                    }
                });
            }
        );
        
        // mmap locate 벤치마크 (한 번 매핑하고 재사용)
        group.bench_with_input(
            BenchmarkId::new("mmap_locate", pattern_len),
            pattern_len,
            |b, _| {
                b.iter(|| {
                    for pattern in patterns {
                        black_box(fm_index_mmap.locate_pattern(pattern));
                    }
                });
            }
        );
    }
    
    group.finish();
    
    // 임시 파일 정리
    if Path::new(&temp_file).exists() {
        fs::remove_file(&temp_file).unwrap();
    }
}

// 다양한 Position과 Block 타입에 대한 벤치마크
pub fn bench_memory_vs_disk_mmap_locate_u32_block2(c: &mut Criterion) {
    bench_memory_vs_disk_mmap_locate::<u32, Block2<u32>>(c);
}

pub fn bench_memory_vs_disk_mmap_locate_u64_block2(c: &mut Criterion) {
    bench_memory_vs_disk_mmap_locate::<u64, Block2<u64>>(c);
}

pub fn bench_memory_vs_disk_mmap_locate_u64_block4(c: &mut Criterion) {
    bench_memory_vs_disk_mmap_locate::<u64, Block4<u64>>(c);
} 