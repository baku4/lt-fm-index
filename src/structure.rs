use std::fmt::Debug;

use crate::{Result, error_msg, Serialize, Deserialize};
use crate::fm_index::{FmIndex, Pattern};

#[derive(Serialize, Deserialize, PartialEq)]
struct LtFmIndex<C> where C: CountArray {
    text_len: u64,
    sa_sampling_ratio: u64,
    suffix_array: Vec<u64>,
    count_array: C,
    bwt: Box<dyn Bwt>,
}

impl<C: CountArray> FmIndex for LtFmIndex<C> {
    fn count(&self, pattern: Pattern) -> u64 {
        if self.count_array.have_klt() {
            self.count_with_klt(pattern)
        } else {
            self.count_without_klt(pattern)
        }
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        if self.count_array.have_klt() {
            self.locate_with_klt(pattern)
        } else {
            self.locate_without_klt(pattern)
        }
    }
}

impl<C: CountArray> LtFmIndex<C> {
    #[inline]
    pub fn count_with_klt(&self, pattern: Pattern) -> u64 {
        let pos_range = self.get_pos_range_with_klt(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    pub fn count_without_klt(&self, pattern: Pattern) -> u64 {
        let pos_range = self.get_pos_range_without_klt(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    pub fn locate_with_klt(&self, pattern: Pattern) -> Vec<u64> {
        let pos_range = self.get_pos_range_with_klt(pattern);
        self.get_location_from_pos_range(pos_range)
    }
    #[inline]
    pub fn locate_without_klt(&self, pattern: Pattern) -> Vec<u64> {
        let pos_range = self.get_pos_range_without_klt(pattern);
        self.get_location_from_pos_range(pos_range)
    }

    fn get_location_from_pos_range(&self, pos_range: (u64, u64)) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sa_sampling_ratio != 0 {
                match self.bwt.get_pre_pos(position, &self.count_array) {
                    Some(v) => {
                        position = v;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    },
                }
                offset += 1;
            }
            let location = self.suffix_array[(position / self.sa_sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }

    fn get_pos_range_with_klt(&self, pattern: &[u8]) -> (u64, u64) {

    }
    fn get_pos_range_without_klt(&self, pattern: &[u8]) -> (u64, u64) {
        let mut idx = pattern.len();
        let chr = pattern[idx-1];
        let mut pos_range = self.get_initial_pos_and_idx_without_klt(chr);
        idx -= 1;
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.next_pos_range_from_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        pos_range
    }

    fn get_initial_pos_and_idx_with_klt(&self, chr: u8) -> (u64, u64) {
        self.kmer_lookup_table.get_initial_pos(chr)
    }
    fn get_initial_pos_and_idx_without_klt(&self, chr: u8) -> (u64, u64) {
        self.count_array.get_initial_pos(chr)
    }
}

trait CountArray {
    fn have_klt(&self) -> bool;
}

trait Bwt {

}

// 
pub type Text = Vec<u8>;

pub struct LtFmIndexOption {
    lookup_table_option: LookupTableOption,
    sa_sampling_ratio: u64,
}
pub struct LookupTableOption {
    kmer_size: usize,
    bit_size: BitSize,
}
pub enum BitSize {
    Bit8,
    Bit16,
}

#[cfg(test)]
mod tests {

}
