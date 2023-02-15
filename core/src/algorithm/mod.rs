use crate::{
    TextLen,
    Text, Pattern,
    Serialize, EndianType, WriteBytesExt, ReadBytesExt,
};
use std::fmt::Debug;

mod suffix_array;
mod count_array;
mod bwm;

use suffix_array::SuffixArray;
use count_array::CountArray;
use bwm::Bwm;
pub use bwm::Block;

#[derive(Clone)]
pub struct RawLtFmIndex<B: Block> {
    text_len: TextLen,
    chr_idx_table: ChrIdxTable,
    suffix_array: SuffixArray,
    count_array: CountArray,
    bwm: Bwm<B>,
}
#[derive(Clone)]
pub struct ChrIdxTable(pub [u8; 256]);
impl ChrIdxTable {
    #[inline]
    fn new_with_counting_chr(characters_by_index: &[&[u8]]) -> (Self, usize) {
        let chr_count = characters_by_index.len();
        let mut table = [(chr_count-1) as u8; 256];
        characters_by_index.iter().enumerate().for_each(|(idx, chr)| {
            chr.iter().for_each(|x| table[*x as usize] = idx as u8);
        });
        (Self(table), chr_count)
    }
    #[inline]
    fn idx_of(&self, chr: u8) -> u8 {
        unsafe { *self.0.get_unchecked(chr as usize) }
    }
}

impl<B: Block> RawLtFmIndex<B> {
    #[inline]
    pub fn count(&self, pattern: Pattern) -> TextLen {
        let pos_range = self.get_pos_range(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    pub fn locate(&self, pattern: Pattern) -> Vec<TextLen> {
        let pos_range = self.get_pos_range(pattern);
        self.get_locations(pos_range)
    }
}

impl<B: Block> RawLtFmIndex<B> {
    // Build
    pub fn new(
        mut text: Text,
        characters_by_index: &[&[u8]],
        suffix_array_sampling_ratio: TextLen,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let text_len = text.len() as TextLen;
        let (chr_idx_table, chr_count) = ChrIdxTable::new_with_counting_chr(characters_by_index);
        let count_array = CountArray::new_while_encoding_text_to_chridxwp(
            &mut text,
            &chr_idx_table,
            chr_count,
            lookup_table_kmer_size,
        );
        let (suffix_array, pidx) = SuffixArray::new_while_bwt(&mut text, suffix_array_sampling_ratio);
        let bwm = Bwm::new(text, pidx, chr_count);
        Self {
            text_len,
            chr_idx_table,
            suffix_array,
            count_array,
            bwm,
        }
    }
    
    // Pos range
    fn get_pos_range(&self, pattern: Pattern) -> (TextLen, TextLen) {
        let (mut pos_range, mut idx) = self.count_array.get_initial_pos_range_and_idx_of_pattern(
            pattern,
            &self.chr_idx_table,
        );
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            idx -= 1;
            let next_chr = pattern[idx];
            pos_range = self.next_pos_range(pos_range, next_chr);
        }
        pos_range
    }
    fn next_pos_range(&self, pos_range: (TextLen, TextLen), chr: u8) -> (TextLen, TextLen) {
        let chridx = self.chr_idx_table.idx_of(chr);
        let precount = self.count_array.get_precount(chridx as usize);
        let start_rank = self.bwm.get_next_rank(pos_range.0, chridx);
        let end_rank = self.bwm.get_next_rank(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }

    // Get index
    fn get_locations(&self, pos_range: (TextLen, TextLen)) -> Vec<TextLen> {
        let mut locations: Vec<TextLen> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut pos in pos_range.0..pos_range.1 {
            let mut offset: TextLen = 0;
            while pos % self.suffix_array.sampling_ratio() != 0 { 
                match self.bwm.get_pre_rank_and_chridx(pos) {
                    Some((rank, chridx)) => {
                        let precount = self.count_array.get_precount(chridx as usize);
                        pos = precount + rank;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    }
                }
                offset += 1;
            }
            let location = self.suffix_array.get_location_of(pos) + offset;
            locations.push(location);
        }
        locations
    }
}

// Serialize
impl Serialize for ChrIdxTable {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        _ = writer.write_all(&self.0)?;
        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read, Self: Sized
    {
        let mut inner = [0; 256];
        reader.read_exact(&mut inner)?;
        Ok(Self(inner))
    }
    fn estimate_size(&self) -> usize {
        256
    }
}
impl<B> RawLtFmIndex<B> where B: Block + bytemuck::Pod {
    pub fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        // text_len
        writer.write_u64::<EndianType>(self.text_len as u64)?;
        // chr_idx_table
        self.chr_idx_table.save_to(&mut writer)?;
        // suffix_array
        self.suffix_array.save_to(&mut writer)?;
        // count_array
        self.count_array.save_to(&mut writer)?;
        // bwm
        self.bwm.save_to(&mut writer)?;
        Ok(())
    }
    pub fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized
    {
        let text_len = reader.read_u64::<EndianType>()? as TextLen;
        let chr_idx_table = ChrIdxTable::load_from(&mut reader)?;
        let suffix_array = SuffixArray::load_from(&mut reader)?;
        let count_array = CountArray::load_from(&mut reader)?;
        let bwm = Bwm::load_from(&mut reader)?;

        Ok(Self {
            text_len,
            chr_idx_table,
            suffix_array,
            count_array,
            bwm,
        })
    }
    pub fn to_be_saved_bytes(&self) -> usize {
        8 // text_len
        + self.chr_idx_table.estimate_size() // chr_idx_table
        + self.suffix_array.estimate_size() // suffix_array
        + self.count_array.estimate_size() // count_array
        + self.bwm.estimate_size() // bwm
    }
}

// Debug features
impl<B: Block> Debug for RawLtFmIndex<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LtFmIndex")
            .field("bit_size_for_position", &TextLen::BITS)
            .field("indexed_text_length", &self.indexed_text_length())
            .field("characters_count", &self.characters_count())
            .field("lookup_table_kmer_size", &self.lookup_table_kmer_size())
            .field("suffix_array_sampling_ratio", &self.suffix_array_sampling_ratio())
            .finish()
    }
}
impl<B: Block> RawLtFmIndex<B> {
    pub fn indexed_text_length(&self) -> TextLen {
        self.text_len
    }
    pub fn characters_count(&self) -> usize {
        self.bwm.chr_count()
    }
    pub fn lookup_table_kmer_size(&self) -> u32 {
        self.count_array.kmer_size()
    }
    pub fn suffix_array_sampling_ratio(&self) -> TextLen {
        self.suffix_array.sampling_ratio()
    }
}
