use super::{FmIndex, Position, Block};

/// Locate pattern
impl<'a, P: Position, B: Block> FmIndex<'a, P, B> {
    // ================================================
    // Plain text
    pub fn count_pattern(&self, pattern: &[u8]) -> P {
        let pos_range = self.get_pos_range(pattern);
        pos_range.1 - pos_range.0
    }
    pub fn locate_pattern(&self, pattern: &[u8]) -> Vec<P> {
        let pos_range = self.get_pos_range(pattern);
        self.get_locations(pos_range)
    }

    fn get_pos_range(&self, pattern: &[u8]) -> (P, P) {
        let (mut pos_range, mut idx) = self.count_array_view.get_initial_pos_range_and_idx_of_pattern(
            pattern,
            &self.chr_encoding_table,
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
        let chridx = self.chr_encoding_table.idx_of(chr);
        let precount = self.count_array_view.get_precount(chridx as usize);
        let start_rank = self.bwm_view.get_next_rank(pos_range.0, chridx);
        let end_rank = self.bwm_view.get_next_rank(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }

    // Get index
    fn get_locations(&self, pos_range: (P, P)) -> Vec<P> {
        let mut locations: Vec<P> = Vec::with_capacity((pos_range.1 - pos_range.0).as_usize());

        'each_pos: for mut pos in P::as_vec_in_range(&pos_range.0, &pos_range.1) {
            let mut offset: P = P::ZERO;
            while pos % self.suffix_array_view.sampling_ratio() != P::ZERO { 
                match self.bwm_view.get_pre_rank_and_chridx(pos) {
                    Some((rank, chridx)) => {
                        let precount = self.count_array_view.get_precount(chridx as usize);
                        pos = precount + rank;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    }
                }
                offset += P::ONE;
            }
            let location = self.suffix_array_view.get_location_of(pos) + offset;
            locations.push(location);
        }
        locations
    }
}
