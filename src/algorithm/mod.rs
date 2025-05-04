use crate::core::{
    Position,
    errors::BuildError,
};

/// FM-index using lookup table for first k-mer search.
/// This is a space-efficient implementation of the FM-index that uses a lookup table
/// for the first k-mer search to improve performance.
#[derive(Clone, PartialEq, Eq)]
pub struct  LtFmIndex<P: Position, B: Block<P>> {
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
    /// Counts the number of occurrences of a pattern in the indexed text.
    /// 
    /// # Arguments
    /// * `pattern` - The pattern to search for
    /// 
    /// # Returns
    /// The number of occurrences of the pattern in the text
    #[inline]
    pub fn count(&self, pattern: &[u8]) -> P {
        let pos_range = self.get_pos_range(pattern);
        pos_range.1 - pos_range.0
    }

    /// Locates all occurrences of a pattern in the indexed text.
    /// 
    /// # Arguments
    /// * `pattern` - The pattern to search for
    /// 
    /// # Returns
    /// A vector of positions where the pattern occurs in the text
    #[inline]
    pub fn locate(&self, pattern: &[u8]) -> Vec<P> {
        let pos_range = self.get_pos_range(pattern);
        self.get_locations(pos_range)
    }
}

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    /// Builds a new FM-index from the given text.
    /// 
    /// # Arguments
    /// * `text` - The text to index
    /// * `characters_by_index` - The characters to index, in order of their indices
    /// * `suffix_array_sampling_ratio` - The sampling ratio for the suffix array
    /// * `lookup_table_kmer_size` - The size of k-mers to use in the lookup table
    /// 
    /// # Returns
    /// A Result containing the built FM-index or a BuildError if construction fails
    pub fn build<T>(
        mut text: Vec<u8>,
        characters_by_index: &[T],
        suffix_array_sampling_ratio: P,
        lookup_table_kmer_size: u32,
    ) -> Result<Self, BuildError>
    where
        T: AsRef<[u8]>,
    {
        if suffix_array_sampling_ratio == P::ZERO {
            return Err(BuildError::SuffixArraySamplingRatio);
        }
        if lookup_table_kmer_size == 0 {
            return Err(BuildError::LookupTableKmerSize);
        }
        let text_len = P::from_usize(text.len());
        let (chr_idx_table, chr_count) = ChrIdxTable::new_with_counting_chr(characters_by_index);
        if chr_count - 1 > B::MAX_CHR {
            return Err(BuildError::IndexCountOver(B::MAX_CHR, chr_count));
        }
        let count_array = CountArray::new_while_encoding_text_to_chridxwp(
            &mut text,
            &chr_idx_table,
            chr_count,
            lookup_table_kmer_size,
        );
        let (suffix_array, pidx) = SuffixArray::new_while_bwt(&mut text, suffix_array_sampling_ratio);
        let bwm = Bwm::new(text, pidx, chr_count);
        Ok(Self {
            text_len,
            chr_idx_table,
            suffix_array,
            count_array,
            bwm,
        })
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
