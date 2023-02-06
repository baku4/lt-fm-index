use std::marker::PhantomData;

use crate::core::{
    Text, Pattern,
    FmIndexInterface,
    LtFmIndexInterface,
};

mod suffix_array;
mod count_array;
mod bwm;

use suffix_array::SuffixArray;
use count_array::CountArray;
use bwm::{Bwm, BwtBlock, VectorBit};

struct RawLtFmIndex<T: TextEncoder> {
    text_len: u64,
    chr_idx_table: ChrIdxTable,
    suffix_array: SuffixArray,
    count_array: CountArray,
    // bwm: Bwm<T>,
    phantom_data: PhantomData<T>,
}
struct ChrIdxTable([u8; 256]);
impl ChrIdxTable {
    fn new(table: [u8; 256]) -> Self {
        Self(table)
    }
    fn idx_of(&self, chr: u8) -> u8 {
        unsafe {
            *self.0.get_unchecked(chr as usize)
        }
    }
}

trait TextEncoder {
    const CHR_COUNT: usize;
    const WILDCARD: u8;

    type Bwm: BwmInterface;

    // Build
    fn encode_chr_idx_table() -> [u8; 256];
}
trait BwmInterface {
    
}

impl<T: TextEncoder> RawLtFmIndex<T> {
    // Build
    fn new(
        mut text: Text,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: u32,
        chr_idx_table: ChrIdxTable,
        chr_count: usize,
        wildcard_chr: u8,
    ) -> Self {
        let text_len = text.len() as u64;
        let count_array = CountArray::new_and_encode_text(
            &mut text,
            &chr_idx_table,
            chr_count,
            lookup_table_kmer_size,
            wildcard_chr,
        );
        let (suffix_array, pidx) = SuffixArray::new_while_bwt(&mut text, suffix_array_sampling_ratio);
        // let bwt = B::new(text, pidx);
        // Self {
        //     text_len,
        //     suffix_array,
        //     count_array,
        //     bwt
        // }
        // FIXME:
        panic!("")
    }
    // Locate
    fn count(&self, pattern: Pattern) -> u64 {
        let pos_range = self.get_pos_range(pattern);
        pos_range.1 - pos_range.0
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        let pos_range = self.get_pos_range(pattern);
        self.get_locations(pos_range)
    }

    // Pos range
    fn get_pos_range(&self, pattern: Pattern) -> (u64, u64) {
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
    fn next_pos_range(&self, pos_range: (u64, u64), chr: u8) -> (u64, u64) {
        let chridx = self.chr_idx_table.idx_of(chr);
        let precount = self.count_array.get_precount(chridx as usize);
        let start_rank = self.bwm.get_next_rank_of_pos_and_chridx(pos_range.0, chridx);
        let end_rank = self.bwm.get_next_rank_of_pos_and_chridx(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }

    // Get index
    fn get_locations(&self, pos_range: (u64, u64)) -> Vec<u64> {
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut pos in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while pos % self.suffix_array.sampling_ratio() != 0 { 
                match self.bwm.get_pre_chridx_and_rank_of_pos(pos) {
                    Some((chridx, rank)) => {
                        let precount = self.count_array.get_precount(chridx);
                        pos = precount + rank;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    }
                }
                offset += 1;
            }
            let location = self.suffix_array.get_location_of_position(pos) + offset;
            locations.push(location);
        }
        locations
    }
}
