use super::{Text, Pattern, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CountArray<F> where F: Fn(&mut u8) -> usize {
    counts: Vec<u64>,
    chr_encoder: F,
    kmer_lookup_table: Option<KmerLookupTable>,
}

impl<F>  CountArray<F> where F: Fn(&mut u8) -> usize {
    pub fn new_and_encode_text(
        text: &mut Text,
        kmer_size: Option<usize>,
        chr_count: usize,
        chr_encoder: F,
    ) {
        let mut counts: Vec<u64> = vec![0; chr_count];
        
    }
    pub fn get_count_of_chridx(&self, chridx: usize) -> u64 {
        self.counts[chridx]
    }
    pub fn get_chridx_and_count_of_chr(&self, chr: u8) -> (usize, u64) {
        let chridx = self.get_chridx_of_chr(chr);
        let count = self.get_count_of_chridx(chridx);
        (chridx, count)
    }
    pub fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        match &self.kmer_lookup_table {
            Some(klt) => { // have kmer lookup table
                klt.get_pos_range_and_idx_of_pattern(pattern)
            },
            None => { // do nsot have kmer lookup table
                let mut idx = pattern.len() - 1;
                let chr = pattern[idx];
                let chridx = self.get_chridx_of_chr(chr);
                let pos_range = (self.counts[chridx], self.counts[chridx+1]);
                idx -= 1;
                (pos_range, idx)
            }
        }
    }

    fn get_chridx_of_chr(&self, chr: &mut u8) -> usize {
        (self.chr_encoder)(chr)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct KmerLookupTable {
    kmer_size: usize,
    multiplier: Vec<usize>,
    offset: usize,
    table: Vec<u64>,
}

impl KmerLookupTable {
    fn new(text: &mut Text, kmer_size: usize, chr_count: usize, ) {
        let multiplier = Self::get_multiplier(kmer_size, chr_count);
        let offset = kmer_size;
        let klt_length = multiplier[0] * chr_count + 1;
    }
    fn get_multiplier(kmer_size: usize, chr_count: usize) -> Vec<usize> {
        let mut mul_of_pos: usize = 0;
        let mut multiplier: Vec<usize> = (0..kmer_size).map(|position| {
            mul_of_pos = mul_of_pos * chr_count + 1;
            mul_of_pos
        }).collect();
        multiplier.reverse();
        multiplier
    }

    fn get_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        let pattern_len = pattern.len();
        let kmer_size = self.kmer_size;

        if pattern_len <= kmer_size { // if pattern is not longer than kmer
            let start_idx = self.idx_of_sliced_pattern(pattern);
            let end_idx = start_idx + self.idx_offset_of_sliced_pattern_length(pattern_len);
            
            let pos_range = if start_idx == 0 {
                (0, self.table[end_idx])
            } else {
                (self.table[start_idx-1], self.table[end_idx])
            };

            (pos_range, 0)
        } else {
            let pattern_idx = pattern_len - kmer_size;
            let start_idx = self.idx_of_sliced_pattern(&pattern[pattern_idx..]);

            let pos_range = if start_idx == 0 {
                (0, self.table[start_idx])
            } else {
                (self.table[start_idx-1], self.table[start_idx])
            };

            (pos_range, pattern_idx)
        }
    }
    fn idx_offset_of_sliced_pattern_length(&self, sliced_pattern_length: usize) -> usize {
        self.multiplier[sliced_pattern_length - 1] - 1
    }
    fn idx_of_sliced_pattern(&self, sliced_pattern: Pattern) -> usize {
        let sum: usize = sliced_pattern.iter()
            .zip(self.multiplier.iter())
            .map(|(&chr, &multiplier_of_position)| {
                (self.idx_encoder)(chr) * multiplier_of_position
            })
            .sum();
        sum - self.offset
    }
}
