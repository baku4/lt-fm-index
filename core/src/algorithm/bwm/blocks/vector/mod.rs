pub trait Vector:
    Sized
    + std::ops::Not<Output = Self>
    + std::ops::BitAnd<Output = Self>
    + std::ops::Shr<Output = Self>
    + std::ops::ShlAssign<Self>
    + std::ops::ShlAssign<u32>
    + std::ops::ShrAssign<u32>
    + std::ops::AddAssign<Self>
    + num_traits::int::PrimInt
{
    const BLOCK_LEN: u32;
    const ZERO: Self;
    const ONE: Self;
    fn as_u8(self) -> u8;
    fn from_u32(value: u32) -> Self;
}

impl Vector for u32 {
    const BLOCK_LEN: u32 = 32;
    const ZERO: Self = 0_u32;
    const ONE: Self = 1_u32;
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn from_u32(value: u32) -> Self {
        value as Self
    }
}
impl Vector for u64 {
    const BLOCK_LEN: u32 = 64;
    const ZERO: Self = 0_u64;
    const ONE: Self = 1_u64;
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn from_u32(value: u32) -> Self {
        value as Self
    }
}
impl Vector for u128 {
    const BLOCK_LEN: u32 = 128;
    const ZERO: Self = 0_u128;
    const ONE: Self = 1_u128;
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn from_u32(value: u32) -> Self {
        value as Self
    }
}