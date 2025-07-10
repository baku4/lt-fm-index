use zerocopy::IntoBytes;

use crate::core::Position;
use super::ChrEncodingTable;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes)]
#[derive(Debug, Clone, PartialEq, Eq)]
/// A data structure for storing and querying character counts in the FM-index
pub struct CountArrayHeader {
    // Given
    pub chr_count: u32,
    pub lookup_table_kmer_size: u32,
    // Derivatives
    pub count_array_len: u32,
    pub kmer_multiplier_len: u32,
    pub kmer_count_table_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountArrayView<'a, P: Position> {
    // Owned
    count_array: Vec<P>,
    kmer_multiplier: Vec<usize>,
    // Reference
    kmer_count_table: &'a [P],
}

#[derive(thiserror::Error, Debug)]
pub enum CountArrayBuildError {
    #[error("Blob buffer is not accurate")]
    BlobBufferNotAccurate,
}

// ================================================
// Build
// ================================================
impl CountArrayHeader {
    pub fn new(
        chr_count: u32,
        lookup_table_kmer_size: u32,
    ) -> Self {
        // chr_count + 1 (wildcard) + 1 (sentinel) => total number of symbols including wildcard and sentinel
        let total_symbol_count = chr_count + 2; // +1 for wildcard, +1 for sentinel  

        let count_array_len = total_symbol_count;
        let kmer_multiplier_len = lookup_table_kmer_size;
        let kmer_count_table_len = (total_symbol_count).pow(lookup_table_kmer_size) as u64;
        
        Self {
            chr_count,
            lookup_table_kmer_size,
            count_array_len,
            kmer_multiplier_len,
            kmer_count_table_len,
        }
    }
    pub fn count_and_encode_text<P: Position>(
        &self,
        text: &mut Vec<u8>,
        chr_encoding_table: &ChrEncodingTable,
        blob_buffer: &mut [u8],
    ) -> Result<(), CountArrayBuildError> {
        // 1) 초기화
        let total_symbol_count = self.count_array_len as usize;
        let count_array_size = total_symbol_count * std::mem::size_of::<P>();
        let kmer_multiplier_size = self.kmer_multiplier_len as usize * std::mem::size_of::<usize>();
        let kmer_count_table_size = self.kmer_count_table_len as usize * std::mem::size_of::<P>();
        // 현재 blob의 공간이 정확한지 확인
        if blob_buffer.len() != count_array_size + kmer_multiplier_size + kmer_count_table_size {
            return Err(CountArrayBuildError::BlobBufferNotAccurate);
        }

        //  - count array
        let mut count_array = vec![P::ZERO; self.count_array_len as usize];
        //  - kmer multiplier (+ 빠른 위치 검색을 위한 chr 인덱스 계산)
        let kmer_multiplier: Vec<usize> = {
            (0..self.lookup_table_kmer_size).map(|pos| {
                (total_symbol_count).pow(pos)
            }).rev().collect()
        };
        let index_for_each_chr: Vec<usize> = {
            (0..(self.chr_count + 1) as usize).map(|chridx| { // Including wild card (chr_count + 1)
                kmer_multiplier[0] * (chridx + 1)
            }).collect()
        };
        // - kmer count array
        let mut kmer_count_array: &mut [P] = {
            let blob_start_index = count_array_size + kmer_multiplier_size;
            // 0으로 init
            blob_buffer[blob_start_index..].fill(0);

            zerocopy::FromBytes::mut_from_bytes(
                &mut blob_buffer[blob_start_index..]
            ).unwrap()
        };
        
        // 2) Counting
        let mut table_index: usize = 0;
        text.iter_mut().rev().for_each(|chr| {
            let chridx = chr_encoding_table.idx_of(*chr);
            // Transform chr to chridx + 1 (sentinel will be 0 for sorting)
            *chr = chridx + 1;
            // Add count to counts
            count_array[chridx as usize + 1] += P::ONE;
            // Update table_index for kmer_count_array
            table_index /= total_symbol_count;
            table_index += index_for_each_chr[chridx as usize];
            // Add count to lookup table
            kmer_count_array[table_index] += P::ONE;
        });

        accumulate_count_array(&mut count_array);
        accumulate_count_array(&mut kmer_count_array);

        // 3) Write data to blob
        blob_buffer[..count_array_size].copy_from_slice(count_array.as_bytes());
        blob_buffer[count_array_size..count_array_size + kmer_multiplier_size].copy_from_slice(kmer_multiplier.as_bytes());

        Ok(())
    }
}

fn accumulate_count_array<P: Position>(count_array: &mut [P]) {
    let mut accumulated_count = P::ZERO;
    count_array.iter_mut().for_each(|count| {
        accumulated_count += *count;
        *count = accumulated_count;
    });
}

// ================================================
// Locate
// ================================================
impl<'a, P: Position> CountArrayView<'a, P> {
    pub fn get_precount(&self, chridx: usize) -> P {
        self.count_array[chridx]
    }
    pub fn get_initial_pos_range_and_idx_of_pattern(
        &self,
        pattern: &[u8],
        lookup_table_kmer_size: usize,
        chr_idx_table: &ChrEncodingTable,
    ) -> ((P, P), usize) {
        let pattern_len = pattern.len();
        if pattern_len < lookup_table_kmer_size {
            let start_idx = self.get_idx_of_kmer_count_table(pattern, chr_idx_table);
            let gap_btw_unsearched_kmer = self.kmer_multiplier[pattern_len - 1] - 1;
            let end_idx = start_idx + gap_btw_unsearched_kmer;

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[end_idx]);
            (pos_range, 0)
        } else {
            let sliced_pattern = &pattern[pattern.len() - lookup_table_kmer_size ..];
            let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern, chr_idx_table);

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[start_idx]);
            (pos_range, pattern_len - lookup_table_kmer_size)
        }
    }
    fn get_idx_of_kmer_count_table(
        &self,
        sliced_pattern: &[u8],
        chr_encoding_table: &ChrEncodingTable,
    ) -> usize {
        sliced_pattern.iter().zip(self.kmer_multiplier.iter())
            .map(|(&chr, &mul_of_pos)| {
                (chr_encoding_table.idx_of(chr) + 1) as usize * mul_of_pos
            }).sum()
    }
}
