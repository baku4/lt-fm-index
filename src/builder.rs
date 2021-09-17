use crate::{Serialize, Deserialize};
use crate::{FmIndex, Text, Pattern};
use crate::structure::{LtFmIndex, CountArray, Bwt};

const CHR_COUNT: usize = 5;
const CHR_WITH_PIDX_COUNT: usize = CHR_COUNT + 1;

const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const G_UTF8: u8 = 71;
const T_UTF8: u8 = 84;
const NOISE_UTF8: u8 = 95; // '_' in ASCII

const A_IDX: usize = 0;
const C_IDX: usize = 1;
const G_IDX: usize = 2;
const T_IDX: usize = 3;
const NOISE_IDX: usize = 4;




type KmerSize = Option<usize>;
type CountTable = Vec<u64>;
type KmerCountTable = Vec<u64>;
type Multiplier = Vec<usize>;

pub trait CountArrayBuilder: CountArray {
    const CHR_COUNT: usize;
    const CHR_WITH_PIDX_COUNT2: usize = CHR_COUNT + 1;

    fn new_and_encode_text(text: &mut Text, kmer_size: KmerSize) -> Self where Self: Sized{
        let mut count_table: Vec<u64> = vec![0; CHR_WITH_PIDX_COUNT];

        let (kmer_count_table, multiplier) = match kmer_size {
            Some(kmer) => {
                let table_length: usize = (CHR_WITH_PIDX_COUNT).pow(kmer as u32);
                let mut kmer_count_table: Vec<u64> = vec![0; table_length];
                let mut table_index: usize = 0;
        
                let multiplier: Vec<usize> = {
                    (0..kmer as u32).map(|pos| {
                        (CHR_WITH_PIDX_COUNT).pow(pos)
                    }).rev().collect()
                };
        
                let index_for_each_chr: Vec<usize> = {
                    (0..CHR_COUNT).map(|chridx| {
                        multiplier[0] * (chridx + 1)
                    }).collect()
                };
        
                text.iter_mut().rev().for_each(|chr| {
                    let chridx = Self::get_chridx_with_encoding_chr(chr);
                    // Add count to counts
                    count_table[chridx + 1] += 1;
                    // Add count to lookup table
                    table_index /= CHR_WITH_PIDX_COUNT;
                    table_index += index_for_each_chr[chridx];
                    kmer_count_table[table_index] += 1;
                });

                Self::accumulate_count_table(&mut kmer_count_table);

                (kmer_count_table, multiplier)
            },
            None => {
                text.iter_mut().rev().for_each(|chr| {
                    let chridx = Self::get_chridx_with_encoding_chr(chr);
                    // Add count to counts
                    count_table[chridx + 1] += 1
                });
                
                (Vec::new(), Vec::new())
            },
        };

        Self::new(kmer_size, count_table, kmer_count_table, multiplier)
    }
    fn get_initial_pos_range_and_idx_of_pattern(
        kmer_size: &KmerSize,
        count_table: &CountTable,
        kmer_count_table: &KmerCountTable,
        multiplier: &Multiplier,
        pattern: Pattern
    ) -> ((u64, u64), usize) {
        match kmer_size {
            Some(kmer) => { // have kmer lookup table
                let pattern_len = pattern.len();
                if pattern_len < *kmer {
                    let start_idx = Self::get_idx_of_kmer_count_table(multiplier, pattern);
                    let gap_btw_unsearched_kmer = multiplier[pattern_len - 1] - 1;
                    let end_idx = start_idx + gap_btw_unsearched_kmer;

                    let pos_range = (kmer_count_table[start_idx -1], kmer_count_table[end_idx]);
                    (pos_range, 0)
                } else {
                    let sliced_pattern = &pattern[pattern.len()-kmer..];
                    let start_idx = Self::get_idx_of_kmer_count_table(multiplier, sliced_pattern);

                    let pos_range = (kmer_count_table[start_idx -1], kmer_count_table[start_idx]);
                    (pos_range, pattern_len-kmer)
                }
            },
            None => { // do not have kmer lookup table
                let idx = pattern.len() - 1;
                let chr = pattern[idx];
                let chridx = Self::chridx_of_chr(chr);
                let pos_range = (count_table[chridx], count_table[chridx+1]);
                (pos_range, idx)
            }
        }
    }

    fn get_idx_of_kmer_count_table(multiplier: &Multiplier, sliced_pattern: Pattern) -> usize {
        sliced_pattern.iter().zip(multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                Self::chrwpidx_of_chr(chr) * mul_of_pos
            }).sum::<usize>()
    }
    fn accumulate_count_table(count_table: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }

    fn new(
        kmer_size: KmerSize,
        count_table: CountTable,
        kmer_count_table: KmerCountTable,
        multiplier: Multiplier,
    ) -> Self;    
    fn chridx_of_chr(chr: u8) -> usize;
    fn chrwpidx_of_chr(chr: u8) -> usize;
    fn get_chridx_with_encoding_chr(chr: &mut u8) -> usize;
    fn count_of_count_table(&self, chridx: usize) -> usize;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CountArrayNN {
    kmer_size: Option<usize>,
    count_table: Vec<u64>,
    kmer_count_table: Vec<u64>,
    multiplier: Vec<usize>,
}

impl CountArray for CountArrayNN {
    fn new_and_encode_text(text: &mut Text, kmer_size: Option<usize>) -> Self {
        let mut count_table: Vec<u64> = vec![0; CHR_WITH_PIDX_COUNT];

        let (kmer_count_table, multiplier) = match kmer_size {
            Some(kmer) => {
                let table_length: usize = (CHR_WITH_PIDX_COUNT).pow(kmer as u32);
                let mut kmer_count_table: Vec<u64> = vec![0; table_length];
                let mut table_index: usize = 0;
        
                let multiplier: Vec<usize> = {
                    (0..kmer as u32).map(|pos| {
                        (CHR_WITH_PIDX_COUNT).pow(pos)
                    }).rev().collect()
                };
        
                let index_for_each_chr: Vec<usize> = {
                    (0..CHR_COUNT).map(|chridx| {
                        multiplier[0] * (chridx + 1)
                    }).collect()
                };
        
                text.iter_mut().rev().for_each(|chr| {
                    let chridx = Self::get_chridx_with_encoding_chr(chr);
                    // Add count to counts
                    count_table[chridx + 1] += 1;
                    // Add count to lookup table
                    table_index /= CHR_WITH_PIDX_COUNT;
                    table_index += index_for_each_chr[chridx];
                    kmer_count_table[table_index] += 1;
                });

                Self::accumulate_count_table(&mut kmer_count_table);

                (kmer_count_table, multiplier)
            },
            None => {
                text.iter_mut().rev().for_each(|chr| {
                    let chridx = Self::get_chridx_with_encoding_chr(chr);
                    // Add count to counts
                    count_table[chridx + 1] += 1
                });
                
                (Vec::new(), Vec::new())
            },
        };

        Self::accumulate_count_table(&mut count_table);

        Self {
            kmer_size,
            count_table,
            kmer_count_table,
            multiplier,
        }
    }
    fn get_precount_of_chridx(&self, chridx: usize) -> u64 {
        self.count_table[chridx]
    }
    fn get_chridx_and_precount_of_chr(&self, chr: u8) -> (usize, u64) {
        let chridx = Self::chridx_of_chr(chr);
        let precount = self.get_precount_of_chridx(chridx);
        (chridx, precount)
    }
    fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        match self.kmer_size {
            Some(kmer) => { // have kmer lookup table
                let pattern_len = pattern.len();
                if pattern_len < kmer {
                    let start_idx = self.get_idx_of_kmer_count_table(pattern);
                    let gap_btw_unsearched_kmer = self.multiplier[pattern_len - 1] - 1;
                    let end_idx = start_idx + gap_btw_unsearched_kmer;

                    let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[end_idx]);
                    (pos_range, 0)
                } else {
                    let sliced_pattern = &pattern[pattern.len()-kmer..];
                    let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern);

                    let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[start_idx]);
                    (pos_range, pattern_len-kmer)
                }
            },
            None => { // do not have kmer lookup table
                let idx = pattern.len() - 1;
                let chr = pattern[idx];
                let chridx = Self::chridx_of_chr(chr);
                let pos_range = (self.count_table[chridx], self.count_table[chridx+1]);
                (pos_range, idx)
            }
        }
    }
}

impl CountArrayNN {
    fn chridx_of_chr(chr: u8) -> usize {
        match chr {
            A_UTF8 => 0,
            C_UTF8 => 1,
            G_UTF8 => 2,
            T_UTF8 => 3,
            _ => {
                4
            },
        }
    }
    fn chrwpidx_of_chr(chr: u8) -> usize {
        match chr {
            A_UTF8 => 1,
            C_UTF8 => 2,
            G_UTF8 => 3,
            T_UTF8 => 4,
            _ => {
                5
            },
        }
    }
    fn get_chridx_with_encoding_chr(chr: &mut u8) -> usize {
        match *chr {
            A_UTF8 => 0,
            C_UTF8 => 1,
            G_UTF8 => 2,
            T_UTF8 => 3,
            _ => {
                *chr = NOISE_UTF8;
                4
            },
        }
    }

    fn get_idx_of_kmer_count_table(&self, sliced_pattern: Pattern) -> usize {
        sliced_pattern.iter().zip(self.multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                Self::chrwpidx_of_chr(chr) * mul_of_pos
            }).sum::<usize>()
    }
    fn accumulate_count_table(count_table: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_table.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }
}

const BLOCK_CHUNK_SIZE: usize = 64;
const BLOCK_SEG_LEN: u64 = BLOCK_CHUNK_SIZE as u64;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct BwtNN {
    primary_index: u64,
    blocks: Vec<BwtBlockNN>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct BwtBlockNN {
    rank_checkpoint: [u64; CHR_COUNT],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
    third_bwt_vector: u64,
}

impl BwtBlockNN {
    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> {
        let mut chunk_count = bwt_text.len() / BLOCK_CHUNK_SIZE;
        let rem = bwt_text.len() % BLOCK_CHUNK_SIZE;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            BLOCK_CHUNK_SIZE - rem
        };

        let mut rank_checkpoint: [u64; CHR_COUNT] = [0; CHR_COUNT];
        let mut blocks: Vec<Self> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(BLOCK_CHUNK_SIZE).for_each(|string_chunk| {
            let block_checkpoint = rank_checkpoint.clone();

            let mut first_bwt_vector: u64 = 0;
            let mut second_bwt_vector: u64 = 0;
            let mut third_bwt_bector: u64 = 0;

            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_bector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[A_IDX] += 1;
                        third_bwt_bector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[C_IDX] += 1;
                        second_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[G_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_bector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[T_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    _ => { // NOISE
                        rank_checkpoint[NOISE_IDX] += 1;
                    }
                }
            }
            let block = BwtBlockNN {
                rank_checkpoint: block_checkpoint,
                first_bwt_vector: first_bwt_vector,
                second_bwt_vector: second_bwt_vector,
                third_bwt_vector: third_bwt_bector,
            };
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = BwtBlockNN {
                rank_checkpoint: rank_checkpoint,
                first_bwt_vector: 0,
                second_bwt_vector: 0,
                third_bwt_vector: 0,
            };
            blocks.push(last_block);
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.add_offset(last_offset);
        }

        blocks
    }
    fn add_offset(&mut self, offset: usize) {
        self.first_bwt_vector <<= offset;
        self.second_bwt_vector <<= offset;
        self.third_bwt_vector <<= offset;
    }

    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        pos_bit >>= rem;

        let chridx = if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                if self.third_bwt_vector & pos_bit == 0 {
                    NOISE_IDX
                } else {
                    A_IDX
                }
            } else {
                C_IDX
            }
        } else {
            if self.second_bwt_vector & pos_bit == 0 {
                G_IDX
            } else {
                T_IDX
            }
        };

        let rank = self.get_rank_of_chridx_and_rem(chridx, rem);

        (chridx, rank)
    }
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64 {
        let mut rank = self.rank_checkpoint[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    (!self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem
                },
                C_IDX => {
                    (!self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
                },
                G_IDX => {
                    (self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem
                },
                T_IDX => {
                    (self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
                },
                _ => { // NOISE
                    (!self.second_bwt_vector & !self.third_bwt_vector) >> 64_u64-rem
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

impl Bwt for BwtNN {
    fn new(bwt_text: Text, pidx: u64) -> Self {
        let blocks: Vec<BwtBlockNN> = BwtBlockNN::new_with_bwt_text(bwt_text);

        Self {
            primary_index: pidx,
            blocks: blocks,
        }
    }
    fn get_pre_chridx_and_rank_of_pos(&self, mut pos: u64) -> Option<(usize, u64)> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / BLOCK_SEG_LEN;
        let rem = pos % BLOCK_SEG_LEN;

        let (chridx, rank) = self.blocks[quot as usize].get_chridx_and_rank_of_rem(rem);
        Some((chridx, rank))
    }
    fn get_next_rank_of_pos_and_chridx(&self, mut pos: u64, chridx: usize) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / BLOCK_SEG_LEN;
        let rem = pos % BLOCK_SEG_LEN;

        self.blocks[quot as usize].get_rank_of_chridx_and_rem(chridx, rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // For cross check
    use crate_fm_index::converter::RangeConverter;
    use crate_fm_index::suffix_array::SuffixOrderSampler;
    use crate_fm_index::{BackwardSearchIndex, FMIndex};
    use rand::Rng;

    fn get_locations_using_other_crate(text: &Vec<u8>, pattern: &Vec<u8>) -> Vec<u64> {
        let converter = RangeConverter::new(b' ', b'~');
        let sampler = SuffixOrderSampler::new().level(2);
        let index = FMIndex::new(text.clone(), converter, sampler);
        let search = index.search_backward(pattern);
        search.locate()
    }

    // Set data
    fn text_on() -> Vec<u8> {
        "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec()
    }
    fn pattern_on() -> Vec<Vec<u8>> {
        vec!["A", "C", "G", "T", "TA", "AA", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    fn text_nn() -> Vec<u8> {
        "CTCCGTACACCTGTTTCGTATCGGNNNAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCNNNCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACNNNCGGATGNNNCATAGATTTCCCCATTTTGCGTANNNNNNNNNNNNNNNNNNCCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec()
    }
    fn pattern_nn() -> Vec<Vec<u8>> {
        ["A", "C", "G", "T", "N", "GA", "AA", "GN", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    const CHARS: [u8; 5] = [65, 67, 71, 84, 95];
    fn text_rand_on() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(CHARS[rng.gen_range(0..4)]));
        text
    }
    fn text_rand_nn() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(CHARS[rng.gen_range(0..5)]));
        text
    }

    #[test]
    fn test_with_NN() {
        let ssa = 8;
        let kmer = 4;
        let text_count = 50;
        let pattern_len = 10;

        for _ in 0..text_count {
            let text_nn = text_rand_nn();

            let lt_fm_index:LtFmIndex<CountArrayNN, BwtNN> = LtFmIndex::new(
                text_nn.clone(), ssa, Some(kmer),
            );
            
            for l in 1..=pattern_len {
                let pattern_nn = text_nn[..l].to_vec();

                let mut loc_nn_res = lt_fm_index.locate(&pattern_nn);
                loc_nn_res.sort();
                let mut loc_nn_ans = get_locations_using_other_crate(&text_nn, &pattern_nn.to_vec());
                loc_nn_ans.sort();

                println!("{:?} < {:?}", String::from_utf8(text_nn.clone()).unwrap(),  String::from_utf8(pattern_nn).unwrap());
                assert_eq!(loc_nn_res, loc_nn_ans);
            }
        }
    }
}