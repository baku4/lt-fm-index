#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChrIdxTable(pub [u8; 256]);

impl ChrIdxTable {
    #[inline]
    pub fn new_with_counting_chr(characters_by_index: &[&[u8]]) -> (Self, u32) {
        let chr_count = characters_by_index.len() as u32 + 1;
        let mut table = [(chr_count - 1) as u8; 256];
        characters_by_index.iter().enumerate().for_each(|(idx, chr)| {
            chr.iter().for_each(|x| table[*x as usize] = idx as u8);
        });
        (Self(table), chr_count)
    }
    #[inline(always)]
    pub fn idx_of(&self, chr: u8) -> u8 {
        unsafe { *self.0.get_unchecked(chr as usize) }
    }
}

mod serialize;