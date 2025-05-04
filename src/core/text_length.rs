/**
Type for the position in index.
Two types are supported:
  - `u32`
  - `u64`
*/
#[cfg(feature = "async-io")]
pub trait Position:
    Sized
    + Copy
    + Clone
    + Ord + PartialOrd + Eq + PartialEq
    + Send + Sync
    + std::fmt::Debug
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Output = Self>
    + std::cmp::PartialOrd
    + bytemuck::Pod
    + bytemuck::Zeroable
    + capwriter::Save
    + capwriter::Load
    + capwriter::AsyncSave
    + capwriter::AsyncLoad
{
    const ZERO: Self;
    const ONE: Self;
    const BITS: u32;
    fn as_u32(self) -> u32;
    fn from_u32(value: u32) -> Self;
    fn as_u64(self) -> u64;
    fn from_u64(value: u64) -> Self;
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
    fn from_i64(value: i64) -> Self;
    fn as_vec_in_range(from: &Self, to: &Self) -> Vec<Self>;
}
#[cfg(not(feature = "async-io"))]
pub trait Position:
    Sized
    + Copy
    + Clone
    + Ord + PartialOrd + Eq + PartialEq
    + Send + Sync
    + std::fmt::Debug
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Output = Self>
    + std::cmp::PartialOrd
    + bytemuck::Pod
    + bytemuck::Zeroable
    + capwriter::Save
    + capwriter::Load
{
    const ZERO: Self;
    const ONE: Self;
    const BITS: u32;
    fn as_u32(self) -> u32;
    fn from_u32(value: u32) -> Self;
    fn as_u64(self) -> u64;
    fn from_u64(value: u64) -> Self;
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
    fn from_i64(value: i64) -> Self;
    fn as_vec_in_range(from: &Self, to: &Self) -> Vec<Self>;
}

impl Position for u32 {
    const ZERO: Self = 0_u32;
    const ONE: Self = 1_u32;
    const BITS: u32 = Self::BITS;
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as u32
    }
    #[inline(always)]
    fn from_u32(value: u32) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_u64(self) -> u64 {
        self as u64
    }
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_usize(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as Self
    }
    #[inline(always)]
    fn from_i64(value: i64) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_vec_in_range(from: &Self, to: &Self) -> Vec<Self> {
        (*from..*to).collect::<Vec<Self>>()
    }
}
impl Position for u64 {
    const ZERO: Self = 0_u64;
    const ONE: Self = 1_u64;
    const BITS: u32 = Self::BITS;
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as u32
    }
    #[inline(always)]
    fn from_u32(value: u32) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_u64(self) -> u64 {
        self as u64
    }
    #[inline(always)]
    fn from_u64(value: u64) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_usize(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as Self
    }
    #[inline(always)]
    fn from_i64(value: i64) -> Self {
        value as Self
    }
    #[inline(always)]
    fn as_vec_in_range(from: &Self, to: &Self) -> Vec<Self> {
        (*from..*to).collect::<Vec<Self>>()
    }
}
