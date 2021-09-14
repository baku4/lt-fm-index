use super::CountArrayInterface;
use super::{Pattern, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CountArray<F> where F: Fn(u8)->usize {
    chr_encoder: F,
    count_array: Vec<u64>,
    kmer_lookup_table: Option<KmerLookupTable<F>>,
}

impl<F> CountArrayInterface for CountArray<F> where F: Fn(u8)->usize {
    fn get_count_of_chridx(&self, chridx: usize) -> u64 {
        self.count_array[chridx]
    }
    fn get_chridx_and_count_of_chr(&self, chr: u8) -> (usize, u64) {
        let chridx = self.get_chridx_of_chr(chr);
        let count = self.get_count_of_chridx(chridx);
        (chridx, count)
    }
    fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        match &self.kmer_lookup_table {
            Some(klt) => { // have kmer lookup table
                klt.get_pos_range_and_idx_of_pattern(pattern)
            },
            None => { // do nsot have kmer lookup table
                let mut idx = pattern.len() - 1;
                let chr = pattern[idx];
                let chridx = self.get_chridx_of_chr(chr);
                let pos_range = (self.count_array[chridx], self.count_array[chridx+1]);
                idx -= 1;
                (pos_range, idx)
            }
        }
    }
}

impl<F> CountArray<F> where F: Fn(u8)->usize {
    fn get_chridx_of_chr(&self, chr: u8) -> usize {
        (self.chr_encoder)(chr)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct KmerLookupTable<F> where F: Fn(u8)->usize {
    kmer_size: usize,
    idx_formatter: IdxFormatter<F>,
    table: Vec<u64>,
}

impl<F> KmerLookupTable<F> where F: Fn(u8)->usize {
    fn get_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        let pattern_len = pattern.len();
        let kmer_size = self.kmer_size;

        if pattern_len <= kmer_size { // if pattern is not longer than kmer
            let start_idx = self.idx_formatter.idx_of_sliced_pattern(pattern);
            let end_idx = start_idx + self.idx_formatter.idx_offset_of_sliced_pattern_length(pattern_len);
            
            let pos_range = if start_idx == 0 {
                (0, self.table[end_idx])
            } else {
                (self.table[start_idx-1], self.table[end_idx])
            };

            (pos_range, 0)
        } else {
            let pattern_idx = pattern_len - kmer_size;
            let start_idx = self.idx_formatter.idx_of_sliced_pattern(&pattern[pattern_idx..]);

            let pos_range = if start_idx == 0 {
                (0, self.table[start_idx])
            } else {
                (self.table[start_idx-1], self.table[start_idx])
            };

            (pos_range, pattern_idx)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct IdxFormatter<F> where F: Fn(u8)->usize {
    idx_encoder: F,
    multiplier: Vec<usize>,
}

impl<F> IdxFormatter<F> where F: Fn(u8)->usize {
    fn idx_of_sliced_pattern(&self, sliced_pattern: Pattern) -> usize {
        let sum: usize = sliced_pattern.iter()
            .zip(self.multiplier.iter())
            .map(|(&chr, &multiplier_of_position)| {
                (self.idx_encoder)(chr) * multiplier_of_position
            })
            .sum();
        sum - self.multiplier[0]
    }
    fn idx_offset_of_sliced_pattern_length(&self, sliced_pattern_length: usize) -> usize {
        self.multiplier[sliced_pattern_length - 1] - 1
    }
}