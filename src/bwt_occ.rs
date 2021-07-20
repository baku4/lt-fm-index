#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

enum BwtSegSize {
    _32,
    _64,
    _128,
    _256,
}

#[cfg(target_arch = "x86")]
type bits_encode = std::arch::x86::__m256i;
#[cfg(target_arch = "x86_64")]
type bits_encode = std::arch::x86_64::__m256i;

#[repr(align(64))]
struct BwtBlock_32 {
    rank_checkpoint: [u64; 4],
    first_bits: u32,
    second_bits: u32,
}

#[repr(align(64))]
struct BwtBlock_64 {
    rank_checkpoint: [u64; 4],
    first_bits: u64,
    second_bits: u64,
}


struct BwtBlock_128 {
    rank_checkpoint: [u64; 4],
    first_bits: u64,
    second_bits: u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
struct BwtBlock_256 {
    rank_checkpoint: [u64; 4],
    first_bits: bits_encode,
    second_bits: bits_encode,
}

struct Bwt_256 {
    blocks: Vec<BwtBlock_256>,
}