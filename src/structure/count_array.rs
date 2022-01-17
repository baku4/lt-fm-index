use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    CountArrayConstructor, CountArrayInterface,
};

use std::marker::PhantomData;


// CountArray Structure

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "CountArray")]
pub struct CountArrayPreBuild<E: TextEncoder> {
    kmer_size: usize,
    count_table: Vec<u64>,
    kmer_count_table: Vec<u64>,
    multiplier: Vec<usize>,
    text_encoder: PhantomData<E>,
}



// CountArray Implementations

impl<E> CountArrayConstructor for CountArrayPreBuild<E> where
    E: TextEncoder,
{
    fn new_and_encode_text(text: &mut Text, kmer_size: usize) -> Self {
        let chr_count = E::CHR_COUNT;
        let chr_with_pidx_count = chr_count + 1;
        let get_chridx_with_encoding_chr = E::get_chridx_with_encoding_chr;

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
            text_encoder: PhantomData,
        }
    }
}

impl<E> CountArrayPreBuild<E> where
    E: TextEncoder,
{
    fn accumulate_count_table(count_table: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }
}

impl<E> CountArrayInterface for CountArray<E> where
    E: TextEncoder,
{
    fn get_precount_of_chridx(&self, chridx: usize) -> u64 {
        self.count_table[chridx]
    }
    fn get_chridx_and_precount_of_chr(&self, chr: u8) -> (usize, u64) {
        let chridx = E::chridx_of_chr(chr);
        let precount = self.get_precount_of_chridx(chridx);
        (chridx, precount)
    }
    fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        let pattern_len = pattern.len();
        if pattern_len < self.kmer_size as usize {
            let start_idx = self.get_idx_of_kmer_count_table(pattern);
            let gap_btw_unsearched_kmer = self.multiplier[pattern_len - 1] - 1;
            let end_idx = start_idx + gap_btw_unsearched_kmer;

            let pos_range = (self.kmer_count_table[start_idx as usize -1], self.kmer_count_table[end_idx as usize]);
            (pos_range, 0)
        } else {
            let sliced_pattern = &pattern[pattern.len() - self.kmer_size as usize..];
            let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern);

            let pos_range = (self.kmer_count_table[start_idx as usize -1], self.kmer_count_table[start_idx as usize]);
            (pos_range, pattern_len - self.kmer_size as usize)
        }
    }
    fn kmer_size(&self) -> usize {
        self.kmer_size as usize
    }
}

impl<E> CountArray<E> where
    E: TextEncoder,
{
    fn get_idx_of_kmer_count_table(&self, sliced_pattern: Pattern) -> u32 {
        sliced_pattern.iter().zip(self.multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                E::chrwpidx_of_chr(chr) * mul_of_pos
            }).sum()
    }
}



// TextEncoder Requirements

pub trait TextEncoder {
    const CHR_COUNT: usize;

    fn get_chridx_with_encoding_chr(unencoded_chr_utf8: &mut u8) -> usize;
    fn chridx_of_chr(chr: u8) -> usize;
    fn chrwpidx_of_chr(chr: u8) -> u32;
}
