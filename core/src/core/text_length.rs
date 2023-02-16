pub trait TextLength:
    Sized
    + Copy
    + Clone
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::Sub<Output = Self>
    + std::cmp::PartialOrd
{
    const ZERO: Self;
    const ONE: Self;
    fn as_u32(self) -> u32;
    fn from_u32(value: u32) -> Self;
    fn as_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
}
impl TextLength for u32 {
    const ZERO: Self = 0_u32;
    const ONE: Self = 1_u32;
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as u32
    }
    #[inline(always)]
    fn from_u32(value: u32) -> Self {
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
}
impl TextLength for u64 {
    const ZERO: Self = 0_u64;
    const ONE: Self = 1_u64;
    #[inline(always)]
    fn as_u32(self) -> u32 {
        self as u32
    }
    #[inline(always)]
    fn from_u32(value: u32) -> Self {
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
}
