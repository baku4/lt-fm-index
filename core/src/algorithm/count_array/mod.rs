use crate::core::{
    TextLength
};
use super::ChrIdxTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountArray<T: TextLength> {
    kmer_size: u32,
    count_table: Vec<T>,
    kmer_count_table: Vec<T>,
    multiplier: Vec<usize>,
}

impl<T: TextLength> CountArray<T> {
    // Build
    pub fn new_while_encoding_text_to_chridxwp(
        text: &mut Vec<u8>,
        chr_idx_table: &ChrIdxTable,
        chr_count: usize,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let chr_with_pidx_count = chr_count + 1;
        let mut count_table: Vec<T> = vec![T::ZERO; chr_with_pidx_count];

        let (kmer_count_table, multiplier) = {
            let table_length: usize = (chr_with_pidx_count).pow(lookup_table_kmer_size);
            let mut kmer_count_table: Vec<T> = vec![T::ZERO; table_length];
            let mut table_index: usize = 0;
    
            let multiplier: Vec<usize> = {
                (0..lookup_table_kmer_size).map(|pos| {
                    (chr_with_pidx_count).pow(pos)
                }).rev().collect()
            };
    
            let index_for_each_chr: Vec<usize> = {
                (0..chr_count).map(|chridx| {
                    multiplier[0] * (chridx + 1)
                }).collect()
            };
    
            text.iter_mut().rev().for_each(|chr| {
                let chridx = chr_idx_table.idx_of(*chr);
                *chr = chridx + 1;
                // Add count to counts
                count_table[chridx as usize + 1] += T::ONE;
                // Add count to lookup table
                table_index /= chr_with_pidx_count;
                table_index += index_for_each_chr[chridx as usize];
                kmer_count_table[table_index] += T::ONE;
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
    fn accumulate_count_table(count_table: &mut [T]) {
        let mut accumed_count = T::ZERO;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }
    
    // Locate
    pub fn get_precount(&self, chridx: usize) -> T {
        self.count_table[chridx]
    }
    pub fn get_initial_pos_range_and_idx_of_pattern(
        &self,
        pattern: &[u8],
        chr_idx_table: &ChrIdxTable,
    ) -> ((T, T), usize) {
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

// impl Serialize for CountArray {
//     fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
//         W: std::io::Write,
//     {
//         // kmer_size
//         writer.write_u64::<EndianType>(self.kmer_size as u64)?;

//         // count_table
//         self.count_table.save_to(&mut writer)?;

//         // kmer_count_table
//         self.kmer_count_table.save_to(&mut writer)?;

//         // multiplier
//         self.multiplier.save_to(&mut writer)?;

//         Ok(())
//     }
//     fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
//         R: std::io::Read,
//         Self: Sized,
//     {
//         // kmer_size
//         let kmer_size = reader.read_u32::<EndianType>()?;

//         // count_table
//         let count_table = Vec::<TextLength>::load_from(&mut reader)?;

//         // kmer_count_table
//         let kmer_count_table = Vec::<TextLength>::load_from(&mut reader)?;

//         // multiplier
//         let multiplier = Vec::<usize>::load_from(&mut reader)?;

//         Ok(Self {
//             kmer_size,
//             count_table,
//             kmer_count_table,
//             multiplier,
//         })
//     }
//     fn estimate_size(&self) -> usize {
//         4 // kmer_size
//         + self.count_table.size_of() // count_table
//         + self.kmer_count_table.size_of() // kmer_count_table
//         + self.multiplier.size_of() // multiplier
//     }
// }
