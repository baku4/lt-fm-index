use crate::structure::{LtFmIndex, CountArray, Bwt};
use crate::{Result, error_msg};
use crate::{Text, Pattern};

use super::use_case::*;

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

pub struct LtFmIndexConfig {
    /// Kmer size for kmer lookup table
    kmer_size: Option<usize>,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
}

impl LtFmIndexConfig {
    const DEFAULT_SA_SAMPLING_RATIO: u64 = 2;

    pub fn change_kmer_size(mut self, kmer_size: usize) -> Result<Self> {
        let max_kmer = POINTER_WIDTH / 2;
        
        if kmer_size < 2 {
            error_msg!("The size of the kmer cannot be less than 2")
        } else if kmer_size > max_kmer {
            error_msg!("The size of the kmer cannot be greater than {} which is limited to half of pointer width({} bits) of target system", max_kmer, POINTER_WIDTH);
        } else {
            self.kmer_size = Some(kmer_size);
            Ok(self)
        }
    }
    pub fn not_use_kmer_lookup_table(mut self) -> Self {
        self.kmer_size = None;
        self
    }
    pub fn change_suffix_array_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Result<Self> {
        if sa_sampling_ratio < 1 {
            error_msg!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            Ok(self)
        }
    }

}

enum TextType {
    OnlyNucleotide,
    NucleotideWithNoise,
    OnlyAminoacid,
    AminoacidWithNoise,
}

impl TextType {
    fn default_kmer_size(&self) -> usize {
        match self {
            Self::OnlyNucleotide => 10, // 5^10 * 8 = 78,125,000 bytes
            Self::NucleotideWithNoise => 9, // 6^9 * 8 = 80,621,568 bytes
            Self::OnlyAminoacid => 5, // 21^5 * 8 = 32,672,808 bytes
            Self::AminoacidWithNoise => 5, // 22^5 * 8 = 41,229,056 bytes
        }
    }
}

enum BwtSegSize {
    Size64,
    Size128,
}
