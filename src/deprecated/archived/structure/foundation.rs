use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize, CheckBytes,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

mod suffix_array;
mod count_array;
mod bwt;

// Defined
use suffix_array::SuffixArrayPreBuild;
// To Define (Requirements)
pub use count_array::TextEncoder;
pub use bwt::{BwtBlockConstructor, BwtBlockInterface};
// Type alias
use count_array::CountArrayPreBuild;
use bwt::BwtPreBuild;
pub type RawLtFmIndexShortPreBuild<E, W> = RawLtFmIndexPreBuild<CountArrayPreBuild<E>, BwtPreBuild<W>>;
pub type RawLtFmIndexShort<E, W> = RawLtFmIndex<CountArrayPreBuild<E>, BwtPreBuild<W>>;

// LtFmIndex Structure

#[derive(Archive, Serialize, Deserialize, Clone)]
#[archive_attr(derive(CheckBytes))]
#[archive(archived = "RawLtFmIndex")]
pub struct RawLtFmIndexPreBuild<C: CountArrayConstructor, B: BwtConstructor> {
    text_len: u64,
    suffix_array: SuffixArrayPreBuild,
    count_array: C,
    bwt: B,
}

// LtFmIndex Implementations

impl<C,B> LtFmIndexConstructor for RawLtFmIndexPreBuild<C, B> where
    C: CountArrayConstructor,
    B: BwtConstructor,
{
    fn new(
        mut text: Text,
        sa_sampling_ratio: u64,
        kmer_size: usize,
    ) -> Self {
        let text_len = text.len() as u64;
        let count_array = CountArrayConstructor::new_and_encode_text(&mut text, kmer_size);
        let (suffix_array, pidx) = SuffixArrayPreBuild::new_while_bwt(&mut text, sa_sampling_ratio);
        let bwt = BwtConstructor::new(text, pidx);
        Self {
            text_len,
            suffix_array,
            count_array,
            bwt
        }
    }
}

impl<C, B> LtFmIndexInterface for RawLtFmIndex<C, B> where
    C: CountArrayConstructor + Archive,
    C::Archived: CountArrayInterface,
    B: BwtConstructor + Archive,
    B::Archived: BwtInterface,
{
    #[inline]
    fn count(&self, pattern: Pattern) -> u64 {
        let pos_range = self.get_pos_range_of_pattern(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        let pos_range = self.get_pos_range_of_pattern(pattern);
        self.get_location_from_pos_range(pos_range)
    }
}

impl<C, B> RawLtFmIndex<C, B> where
    C: CountArrayConstructor + Archive,
    C::Archived: CountArrayInterface,
    B: BwtConstructor + Archive,
    B::Archived: BwtInterface,
{
    // For interface implementation
    fn get_location_from_pos_range(&self, pos_range: (u64, u64)) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut pos in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while pos % self.suffix_array.sampling_ratio != 0 { 
                match self.bwt.get_pre_chridx_and_rank_of_pos(pos) {
                    Some((chridx, rank)) => {
                        let precount = self.count_array.get_precount_of_chridx(chridx);
                        pos = precount + rank;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    }
                }
                offset += 1;
            }
            let location = self.suffix_array.array[(pos / self.suffix_array.sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }
    fn get_pos_range_of_pattern(&self, pattern: &[u8]) -> (u64, u64) {
        let (mut pos_range, mut idx) = self.count_array.get_initial_pos_range_and_idx_of_pattern(pattern);
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            idx -= 1;
            let next_chr = pattern[idx];
            pos_range = self.get_next_pos_range_of_pos_range_and_chr(pos_range, next_chr);
        }
        pos_range
    }
    fn get_next_pos_range_of_pos_range_and_chr(&self, pos_range: (u64, u64), chr: u8) -> (u64, u64) {
        let (chridx, precount) = self.count_array.get_chridx_and_precount_of_chr(chr);
        let start_rank = self.bwt.get_next_rank_of_pos_and_chridx(pos_range.0, chridx);
        let end_rank = self.bwt.get_next_rank_of_pos_and_chridx(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }

    // For getting information
    pub fn suffix_array_sampling_ratio(&self) -> u64 {
        self.suffix_array.sampling_ratio
    }
    pub fn lookup_table_kmer_size(&self) -> usize {
        self.count_array.kmer_size()
    }
}



// CountArray Requirements

pub trait CountArrayConstructor {
    fn new_and_encode_text(text: &mut Text, kmer_size: usize) -> Self;
}
pub trait CountArrayInterface {
    fn get_precount_of_chridx(&self, chridx: usize) -> u64;
    fn get_chridx_and_precount_of_chr(&self, chr: u8) -> (usize, u64);
    fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize);
    fn kmer_size(&self) -> usize;
}



// Bwt Requirements

pub trait BwtConstructor {
    fn new(bwt_text: Text, pidx: u64) -> Self;
}
pub trait BwtInterface {
    fn get_pre_chridx_and_rank_of_pos(&self, pos: u64) -> Option<(usize, u64)>;
    fn get_next_rank_of_pos_and_chridx(&self, pos: u64, chridx: usize) -> u64;
}
