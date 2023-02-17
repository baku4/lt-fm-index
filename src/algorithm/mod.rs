use crate::core::{
    Position,
};

#[derive(Clone)]
pub struct LtFmIndex<P: Position, B: Block<P>> {
    text_len: P,
    chr_idx_table: ChrIdxTable,
    suffix_array: SuffixArray<P>,
    count_array: CountArray<P>,
    bwm: Bwm<P, B>,
}

mod chr_idx_table;
use chr_idx_table::ChrIdxTable;
mod suffix_array;
use suffix_array::SuffixArray;
mod count_array;
use count_array::CountArray;
mod bwm;
use bwm::Bwm;
pub use bwm::{Block, blocks};

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    #[inline]
    pub fn count(&self, pattern: &[u8]) -> P {
        let pos_range = self.get_pos_range(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    pub fn locate(&self, pattern: &[u8]) -> Vec<P> {
        let pos_range = self.get_pos_range(pattern);
        self.get_locations(pos_range)
    }
}

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    // Build
    pub fn new(
        mut text: Vec<u8>,
        characters_by_index: &[&[u8]],
        suffix_array_sampling_ratio: P,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let text_len = P::from_usize(text.len());
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
    fn get_pos_range(&self, pattern: &[u8]) -> (P, P) {
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
    fn next_pos_range(&self, pos_range: (P, P), chr: u8) -> (P, P) {
        let chridx = self.chr_idx_table.idx_of(chr);
        let precount = self.count_array.get_precount(chridx as usize);
        let start_rank = self.bwm.get_next_rank(pos_range.0, chridx);
        let end_rank = self.bwm.get_next_rank(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }

    // Get index
    fn get_locations(&self, pos_range: (P, P)) -> Vec<P> {
        let mut locations: Vec<P> = Vec::with_capacity((pos_range.1 - pos_range.0).as_usize());

        'each_pos: for mut pos in P::as_vec_in_range(&pos_range.0, &pos_range.1) {
            let mut offset: P = P::ZERO;
            while pos % self.suffix_array.sampling_ratio() != P::ZERO { 
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
                offset += P::ONE;
            }
            let location = self.suffix_array.get_location_of(pos) + offset;
            locations.push(location);
        }
        locations
    }
}

mod features;
