use crate::proto::{BwtBlock, BwtProto};
use crate::structure::{LtFmIndex, CountArray, Bwt};
use crate::{FmIndex, Result, error_msg};
use crate::{Text, Pattern};

use super::use_case::*;

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

use std::marker::PhantomData;

struct LtFmIndexConfig<C: CountArray, B: BwtBlock> {
    /// Type of text
    text_type: TextType,
    /// Kmer size for lookup table
    kmer_size: Option<usize>,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
    // Type marker
    _marker: (PhantomData<C>, PhantomData<B>)
}

enum TextType {
    NucleotideOnly,
    NucleotideWithNoise,
    AminoacidOnly,
    AminoacidWithNoise,
}

impl TextType {
    fn default_kmer_size(&self) -> usize {
        match self {
            Self::NucleotideOnly => 10, // 5^10 * 8 = 78,125,000 bytes
            Self::NucleotideWithNoise => 9, // 6^9 * 8 = 80,621,568 bytes
            Self::AminoacidOnly => 5, // 21^5 * 8 = 32,672,808 bytes
            Self::AminoacidWithNoise => 5, // 22^5 * 8 = 41,229,056 bytes
        }
    }
}


impl<C: CountArray, B: BwtBlock> LtFmIndexConfig<C, B> {
    const DEFAULT_SA_SAMPLING_RATIO: u64 = 2;

    pub fn for_nucleotide() -> Self {
        Self {
            text_type: TextType::NucleotideOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::DEFAULT_SA_SAMPLING_RATIO,
            _marker: (PhantomData, PhantomData),
        }
    }
    pub fn for_aminoacid() -> Self {
        Self {
            text_type: TextType::AminoacidOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::DEFAULT_SA_SAMPLING_RATIO,
            _marker: (PhantomData, PhantomData),
        }
    }
    pub fn with_noise(mut self) -> Self {
        match self.text_type {
            TextType::NucleotideOnly => {
                self.text_type = TextType::NucleotideWithNoise
            },
            TextType::AminoacidOnly => {
                self.text_type = TextType::AminoacidWithNoise
            },
            _ => {},
        }
        self
    }
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
    pub fn change_suffix_array_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Result<Self> {
        if sa_sampling_ratio < 1 {
            error_msg!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            Ok(self)
        }
    }
    pub fn generate(self, text: Text) -> LtFmIndex<C, BwtProto<B>> {
        LtFmIndex::<C, BwtProto<B>>::new(text, self.sa_sampling_ratio, self.kmer_size())
    }

    fn kmer_size(&self) -> usize {
        match self.kmer_size {
            Some(kmer_size) => kmer_size,
            None => self.text_type.default_kmer_size(),
        }
    }

}
