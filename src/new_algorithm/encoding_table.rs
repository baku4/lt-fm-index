use super::Header;

/// A table mapping characters to their indices in the FM-index
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
pub struct ChrEncodingTable([u8; 256]);

impl ChrEncodingTable {
    #[inline]
    pub fn new<T>(characters_by_index: &[T]) -> Self
    where
        T: AsRef<[u8]>,
    {
        let chr_count = characters_by_index.len() as u32;
        let mut table = [chr_count as u8; 256]; // wild card's index is chr_count
        characters_by_index.iter().enumerate().for_each(|(idx, chr)| {
            chr.as_ref().iter().for_each(|x| table[*x as usize] = idx as u8);
        });
        Self(table)
    }
    #[inline(always)]
    pub fn idx_of(&self, chr: u8) -> u8 {
        unsafe { *self.0.get_unchecked(chr as usize) }
    }
}

impl Header for ChrEncodingTable {}