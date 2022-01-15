use crate::{Serialize, Deserialize};
use crate::{Text, Pattern};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CountArrayProto {
    kmer_size: usize,
    count_table: Vec<u64>,
    kmer_count_table: Vec<u64>,
    multiplier: Vec<usize>,
}

impl CountArrayProto {
    pub fn new_and_encode_text<F> (
        text: &mut Text,
        kmer_size: usize,
        chr_count: usize,
        chr_with_pidx_count: usize,
        get_chridx_with_encoding_chr: F,
    ) -> Self where F: Fn(&mut u8) -> usize {
        let mut count_table: Vec<u64> = vec![0; chr_with_pidx_count];

        let (kmer_count_table, multiplier) = {
            let table_length: usize = (chr_with_pidx_count).pow(kmer_size as u32);
            let mut kmer_count_table: Vec<u64> = vec![0; table_length];
            let mut table_index: usize = 0;
    
            let multiplier: Vec<usize> = {
                (0..kmer_size as u32).map(|pos| {
                    (chr_with_pidx_count).pow(pos)
                }).rev().collect()
            };
    
            let index_for_each_chr: Vec<usize> = {
                (0..chr_count).map(|chridx| {
                    multiplier[0] * (chridx + 1)
                }).collect()
            };
    
            text.iter_mut().rev().for_each(|chr| {
                let chridx = get_chridx_with_encoding_chr(chr);
                // Add count to counts
                count_table[chridx + 1] += 1;
                // Add count to lookup table
                table_index /= chr_with_pidx_count;
                table_index += index_for_each_chr[chridx];
                kmer_count_table[table_index] += 1;
            });

            Self::accumulate_count_table(&mut kmer_count_table);

            (kmer_count_table, multiplier)
        };

        Self::accumulate_count_table(&mut count_table);

        Self {
            kmer_size,
            count_table,
            kmer_count_table,
            multiplier,
        }
    }
    pub fn get_precount_of_chridx(&self, chridx: usize) -> u64 {
        self.count_table[chridx]
    }
    pub fn get_chridx_and_precount_of_chr<F>(
        &self,
        chr: u8,
        chridx_of_chr: F,
    ) -> (usize, u64) where F: Fn(u8) -> usize {
        let chridx = chridx_of_chr(chr);
        let precount = self.get_precount_of_chridx(chridx);
        (chridx, precount)
    }
    pub fn get_initial_pos_range_and_idx_of_pattern<F>(
        &self,
        pattern: Pattern,
        chrwpidx_of_chr: F,
    ) -> ((u64, u64), usize) where F: Fn(u8) -> usize {
        let pattern_len = pattern.len();
        if pattern_len < self.kmer_size {
            let start_idx = self.get_idx_of_kmer_count_table(pattern, chrwpidx_of_chr);
            let gap_btw_unsearched_kmer = self.multiplier[pattern_len - 1] - 1;
            let end_idx = start_idx + gap_btw_unsearched_kmer;

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[end_idx]);
            (pos_range, 0)
        } else {
            let sliced_pattern = &pattern[pattern.len()-self.kmer_size..];
            let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern, chrwpidx_of_chr);

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[start_idx]);
            (pos_range, pattern_len - self.kmer_size)
        }
    }

    fn get_idx_of_kmer_count_table<F> (
        &self,
        sliced_pattern: Pattern,
        chrwpidx_of_chr: F,
    ) -> usize where F: Fn(u8) -> usize {
        sliced_pattern.iter().zip(self.multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                chrwpidx_of_chr(chr) * mul_of_pos
            }).sum::<usize>()
    }
    fn accumulate_count_table(count_table: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }

    pub fn kmer_size(&self) -> usize {
        self.kmer_size
    }
}