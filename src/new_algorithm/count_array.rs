use zerocopy::IntoBytes;

use crate::core::Position;
use super::{ChrEncodingTable, Header, View, calculate_byte_size_with_padding};

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
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
    // From header
    lookup_table_kmer_size: usize,
    // Owned
    count_array: Vec<P>,
    kmer_multiplier: Vec<usize>,
    // Reference
    kmer_count_table: &'a [P],
}

impl CountArrayHeader {
    fn count_array_raw_size<P: Position>(&self) -> usize {
        self.count_array_len as usize * std::mem::size_of::<P>()
    }
    fn count_array_aligned_size<P: Position>(&self) -> usize {
        calculate_byte_size_with_padding(self.count_array_raw_size::<P>())
    }
    fn kmer_multiplier_raw_size(&self) -> usize {
        self.kmer_multiplier_len as usize * std::mem::size_of::<usize>()
    }
    fn kmer_multiplier_aligned_size(&self) -> usize {
        calculate_byte_size_with_padding(self.kmer_multiplier_raw_size())
    }
    fn kmer_count_table_raw_size<P: Position>(&self) -> usize {
        self.kmer_count_table_len as usize * std::mem::size_of::<P>()
    }
    fn kmer_count_table_aligned_size<P: Position>(&self) -> usize {
        calculate_byte_size_with_padding(self.kmer_count_table_raw_size::<P>())
    }
}

impl Header for CountArrayHeader {}

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
        blob: &mut [u8],
    ) {
        // 1) Init
        let total_symbol_count = self.count_array_len as usize;
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
            let blob_start_index = self.count_array_aligned_size::<P>() + self.kmer_multiplier_aligned_size();
            let blob_end_index = blob_start_index + self.kmer_count_table_raw_size::<P>();
            // 0으로 init
            let body = &mut blob[blob_start_index..blob_end_index];
            body.fill(0);

            zerocopy::FromBytes::mut_from_bytes(body).unwrap()
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
        blob[
            ..self.count_array_raw_size::<P>()
        ].copy_from_slice(count_array.as_bytes());
        blob[
            self.count_array_aligned_size::<P>()
            ..self.count_array_aligned_size::<P>() + self.kmer_multiplier_raw_size()
        ].copy_from_slice(kmer_multiplier.as_bytes());
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
// Load
// ================================================

impl<'a, P: Position> View<'a> for CountArrayView<'a, P> {
    type Header = CountArrayHeader;

    fn aligned_body_size(header: &Self::Header) -> usize {
        header.count_array_aligned_size::<P>()
        + header.kmer_multiplier_aligned_size()
        + header.kmer_count_table_aligned_size::<P>()
    }

    fn load_from_body(
        header: &Self::Header,
        body_blob: &'a [u8],
    ) -> Self {
        let mut body_start_index = 0;
        let mut body_end_index = header.count_array_raw_size::<P>();
        let mut next_body_start_index = header.count_array_aligned_size::<P>();

        // Count array
        let count_array_bytes = &body_blob[body_start_index..body_end_index];
        let count_array: &[P] = zerocopy::FromBytes::ref_from_bytes(count_array_bytes).unwrap();

        // Kmer multiplier
        body_start_index = next_body_start_index;
        body_end_index = body_start_index + header.kmer_multiplier_raw_size();
        next_body_start_index = body_start_index + header.kmer_multiplier_aligned_size();
        let kmer_multiplier_bytes = &body_blob[body_start_index..body_end_index];
        let kmer_multiplier: &[usize] = zerocopy::FromBytes::ref_from_bytes(kmer_multiplier_bytes).unwrap();

        // Kmer count table
        body_start_index = next_body_start_index;
        body_end_index = body_start_index + header.kmer_count_table_raw_size::<P>();
        let kmer_count_table_bytes = &body_blob[body_start_index..body_end_index];
        let kmer_count_table: &'a [P] = zerocopy::FromBytes::ref_from_bytes(kmer_count_table_bytes).unwrap();
        
        Self {
            lookup_table_kmer_size: header.lookup_table_kmer_size as usize,
            count_array: count_array.to_vec(),
            kmer_multiplier: kmer_multiplier.to_vec(),
            kmer_count_table,
        }
    }
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
        chr_encoding_table: &ChrEncodingTable,
    ) -> ((P, P), usize) {
        let pattern_len = pattern.len();
        if pattern_len < self.lookup_table_kmer_size {
            let start_idx = self.get_idx_of_kmer_count_table(pattern, chr_encoding_table);
            let gap_btw_unsearched_kmer = self.kmer_multiplier[pattern_len - 1] - 1;
            let end_idx = start_idx + gap_btw_unsearched_kmer;

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[end_idx]);
            (pos_range, 0)
        } else {
            let sliced_pattern = &pattern[pattern.len() - self.lookup_table_kmer_size ..];
            let start_idx = self.get_idx_of_kmer_count_table(sliced_pattern, chr_encoding_table);

            let pos_range = (self.kmer_count_table[start_idx -1], self.kmer_count_table[start_idx]);
            (pos_range, pattern_len - self.lookup_table_kmer_size)
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
