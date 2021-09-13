use crate::{Result, error_msg, Serialize, Deserialize};
use crate::fm_index::{FmIndex, Pattern};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LtFmIndex {
    text_len: u64,
    count_array: CountArray,
    sampling_ratio: u64,
    suffix_array: SuffixArray,
    bwt: Bwt,
    kmer_lookup_table: Option<KmerLookupTable>,
    bitcount_lookup_table: BitcountLookupTable,
}

impl FmIndex for LtFmIndex {
    fn count(&self, pattern: Pattern) -> u64 {
        match self.kmer_lookup_table {
            Some(_) => self.count_with_klt(pattern),
            None => self.count_without_klt(pattern),
        }
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        match self.kmer_lookup_table {
            Some(_) => self.locate_with_klt(pattern),
            None => self.locate_without_klt(pattern),
        }
    }
}

impl LtFmIndex {
    pub fn count_with_klt(&self, pattern: Pattern) -> u64 {
        let pos_range = self.get_pos_range_with_klt();
        
    }
    pub fn count_without_klt(&self, pattern: Pattern) -> u64 {

    }
    pub fn locate_with_klt(&self, pattern: Pattern) -> Vec<u64> {

    }
    pub fn locate_without_klt(&self, pattern: Pattern) -> Vec<u64> {
        
    }

    fn get_pos_range_with_klt(&self, pattern: &[u8]) -> (u64, u64) {

    }
    fn get_pos_range_without_klt(&self, pattern: &[u8]) -> (u64, u64) {
        let mut idx = pattern.len();
        let chr = pattern[idx-1];
        let mut pos_range = self.get_initial_pos_without_klt(chr);
        idx -= 1;
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.next_pos_range_from_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        pos_range
    }

    fn get_initial_pos_with_klt(&self, chr: u8) -> (u64, u64) {
        self.kmer_lookup_table.get_initial_pos(chr)
    }
    fn get_initial_pos_without_klt(&self, chr: u8) -> (u64, u64) {
        self.count_array.get_initial_pos(chr)
    }
}



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