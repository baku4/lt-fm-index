use crate::{Serialize, Deserialize};
use crate::fm_index::{FmIndex, Pattern};

pub mod suffix_array;
pub mod count_array;
pub mod bwt;

use suffix_array::get_suffix_array_and_pidx_with_bw_transforming;
use count_array::CountArray;
use bwt::Bwt;

use self::bwt::BwtBlockInterface;

type Text = Vec<u8>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LtFmIndex<F, B> where F: Fn(u8) -> usize, B: BwtBlockInterface {
    text_len: u64,
    sa_sampling_ratio: u64,
    suffix_array: Vec<u64>,
    count_array: CountArray<F>,
    bwt: Bwt<B>,
}

impl<F, B> FmIndex for LtFmIndex<F, B> where F: Fn(u8) -> usize, B: BwtBlockInterface {
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

impl<F, B> LtFmIndex<F, B> where F: Fn(u8) -> usize, B: BwtBlockInterface {
    fn new(
        mut text: Text,
        sa_sampling_ratio: u64,
    ) {
        let text_len = text.len() as u64;
        // let count_array = CountArray::new_and_encode_text(&mut text);
        let (suffix_array, pidx) = get_suffix_array_and_pidx_with_bw_transforming(&mut text, sa_sampling_ratio);
        // let bwt = Bwt::new(text, pidx);
    }
    
    fn get_location_from_pos_range(&self, pos_range: (u64, u64)) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut pos in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while pos % self.sa_sampling_ratio != 0 { 
                match self.bwt.get_pre_chridx_and_rank_of_pos(pos) {
                    Some((chridx, rank)) => {
                        let count = self.count_array.get_count_of_chridx(chridx);
                        pos = count + rank;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    }
                }
                offset += 1;
            }
            let location = self.suffix_array[(pos / self.sa_sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }
    fn get_pos_range_of_pattern(&self, pattern: &[u8]) -> (u64, u64) {
        let (mut pos_range, mut idx) = self.count_array.get_initial_pos_range_and_idx_of_pattern(pattern);
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let chr = pattern[idx-1];
            pos_range = self.get_next_pos_range_of_pos_range_and_chr(pos_range, chr);
            idx -= 1;
        }
        pos_range
    }
    fn get_next_pos_range_of_pos_range_and_chr(&self, pos_range: (u64, u64), chr: u8) -> (u64, u64) {
        let (chridx, count) = self.count_array.get_chridx_and_count_of_chr(chr);
        let start_rank = self.bwt.get_next_rank_of_pos_and_chridx(pos_range.0, chridx);
        let end_rank = self.bwt.get_next_rank_of_pos_and_chridx(pos_range.1, chridx);
        (count + start_rank, count + end_rank)
    }
}

// pub trait CountArrayInterface {
//     fn get_count_of_chridx(&self, chridx: usize) -> u64;
//     fn get_chridx_and_count_of_chr(&self, chr: u8) -> (usize, u64);
//     fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize);
// }

// pub trait BwtInterface {
//     fn get_pre_chridx_and_rank_of_pos(&self, pos: u64) -> Option<(usize, u64)>;
//     fn get_next_rank_of_pos_and_chridx(&self, pos: u64, chridx: usize) -> u64;
// }

#[cfg(test)]
mod tests {
    fn test_count() {

    }
    fn test_locate() {
        
    }
}
