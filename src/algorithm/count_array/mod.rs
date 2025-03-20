use crate::core::Position;
use super::ChrIdxTable;

// pub(crate) is used for 'features > locate_from_raw_index.rs'
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountArray<P: Position> {
    pub(crate) kmer_size: u32,
    pub(crate) count_table: Vec<P>,
    pub(crate) kmer_count_table: Vec<P>,
    pub(crate) multiplier: Vec<usize>,
}

impl<P: Position> CountArray<P> {
    // Build
    pub fn new_while_encoding_text_to_chridxwp(
        text: &mut Vec<u8>,
        chr_idx_table: &ChrIdxTable,
        chr_count: u32,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let chr_with_pidx_count = (chr_count + 1) as usize;
        let mut count_table: Vec<P> = vec![P::ZERO; chr_with_pidx_count];

        let (kmer_count_table, multiplier) = {
            let table_length: usize = (chr_with_pidx_count).pow(lookup_table_kmer_size);
            let mut kmer_count_table: Vec<P> = vec![P::ZERO; table_length];
            let mut table_index: usize = 0;
    
            let multiplier: Vec<usize> = {
                (0..lookup_table_kmer_size).map(|pos| {
                    (chr_with_pidx_count).pow(pos)
                }).rev().collect()
            };
    
            let index_for_each_chr: Vec<usize> = {
                (0..chr_count as usize).map(|chridx| {
                    multiplier[0] * (chridx + 1)
                }).collect()
            };
    
            text.iter_mut().rev().for_each(|chr| {
                let chridx = chr_idx_table.idx_of(*chr);
                *chr = chridx + 1;
                // Add count to counts
                count_table[chridx as usize + 1] += P::ONE;
                // Add count to lookup table
                table_index /= chr_with_pidx_count;
                table_index += index_for_each_chr[chridx as usize];
                kmer_count_table[table_index] += P::ONE;
            });

            Self::accumulate_count_table(&mut kmer_count_table);

            (kmer_count_table, multiplier)
        };

        Self::accumulate_count_table(&mut count_table);

        Self {
            kmer_size: lookup_table_kmer_size,
            count_table,
            kmer_count_table,
            multiplier,
        }
    }
    fn accumulate_count_table(count_table: &mut [P]) {
        let mut accumed_count = P::ZERO;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }
    
    // Locate
    pub fn get_precount(&self, chridx: usize) -> P {
        self.count_table[chridx]
    }
    pub fn get_initial_pos_range_and_idx_of_pattern(
        &self,
        pattern: &[u8],
        chr_idx_table: &ChrIdxTable,
    ) -> ((P, P), usize) {
        let pattern_len = pattern.len();
        if pattern_len < self.kmer_size as usize {
            let start_idx = self.get_idx_of_kmer_count_table(pattern, chr_idx_table);
            let gap_btw_unsearched_kmer = self.multiplier[pattern_len - 1] - 1;
            let end_idx = start_idx + gap_btw_unsearched_kmer;

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[end_idx]);
            (pos_range, 0)
        } else {
            let sliced_pattern = &pattern[pattern.len() - self.kmer_size as usize ..];
            let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern, chr_idx_table);

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[start_idx]);
            (pos_range, pattern_len - self.kmer_size as usize)
        }
    }
    fn get_idx_of_kmer_count_table(
        &self,
        sliced_pattern: &[u8],
        chr_idx_table: &ChrIdxTable,
    ) -> usize {
        sliced_pattern.iter().zip(self.multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                (chr_idx_table.idx_of(chr) + 1) as usize * mul_of_pos
            }).sum()
    }

    pub fn kmer_size(&self) -> u32 {
        self.kmer_size
    }
}

mod serialize;
