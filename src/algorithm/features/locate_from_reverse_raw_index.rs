use crate::Position;

use super::{LtFmIndex, CountArray, Block};

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    #[inline]
    /// Returns the decoding table.
    /// The decoding table is a 256-length array that stores the indices for 1-byte characters.
    pub fn decoding_table(&self) -> &[u8; 256] {
        &self.chr_idx_table.0
    }
    #[inline]
    /// Performs the same functionality as [Self::locate], but instead of plain text, it iterates over the text in reverse order and takes a raw index as input using the decoding table (with [Self::decoding_table]).
    pub fn locate_from_raw_index<I: Iterator<Item = u8>>(
        &self,
        raw_index_rev_iter: I,
    ) -> Vec<P> {
        let pos_range = self.get_pos_range_from_raw_index(raw_index_rev_iter);
        self.get_locations(pos_range) // Same as locate
    }
    // Pos range
    #[inline]
    fn get_pos_range_from_raw_index<I: Iterator<Item = u8>>(
        &self,
        mut raw_index_rev_iter: I,
    ) -> (P, P) {
        let mut pos_range = self.count_array.get_initial_pos_range_and_idx_of_pattern_without_chr_idx_table(
            &mut raw_index_rev_iter,
        );
        // LF mapping
        while pos_range.0 < pos_range.1  {
            match raw_index_rev_iter.next() {
                Some(next_chr) => {
                    pos_range = self.next_pos_range_without_chr_idx_table(pos_range, next_chr);
                },
                None => break,
            };
        }
        pos_range
    }
    #[inline]
    fn next_pos_range_without_chr_idx_table(&self, pos_range: (P, P), chridx: u8) -> (P, P) {
        let precount = self.count_array.get_precount(chridx as usize);
        let start_rank = self.bwm.get_next_rank(pos_range.0, chridx);
        let end_rank = self.bwm.get_next_rank(pos_range.1, chridx);
        (precount + start_rank, precount + end_rank)
    }
}

// For Count Array
impl<P: Position> CountArray<P> {
    pub fn get_initial_pos_range_and_idx_of_pattern_without_chr_idx_table<I: Iterator<Item = u8>>(
        &self,
        raw_index_rev_iter: &mut I,
    ) -> (P, P) {
        let mut sliced_pattern_size = 0;
        let mut start_idx= 0;

        while sliced_pattern_size < self.kmer_size {
            match raw_index_rev_iter.next() {
                Some(chridx) => {
                    sliced_pattern_size += 1;
                    start_idx += (chridx + 1) as usize * self.multiplier[
                        self.multiplier.len() - sliced_pattern_size as usize
                    ];
                },
                None => {
                    // The pattern length can be smaller than the k-mer size.
                    // Multiply by chr_with_pidx_count.pow(self.kmer_size - sliced_pattern_size).
                    // Here, chr_with_pidx_count = self.count_table.len().
                    start_idx *= self.count_table.len().pow(self.kmer_size - sliced_pattern_size);

                    let gap_btw_unsearched_kmer = self.multiplier[sliced_pattern_size as usize - 1] - 1;
                    let end_idx = start_idx + gap_btw_unsearched_kmer;

                    let pos_range = (
                        self.kmer_count_table[start_idx -1],
                        self.kmer_count_table[end_idx],
                    );
                    return pos_range
                },
            };
        }

        let pos_range = (
            self.kmer_count_table[start_idx -1],
            self.kmer_count_table[start_idx],
        );
        pos_range
    }
}
