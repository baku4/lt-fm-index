use std::marker::PhantomData;

use super::{
    // traits
    Position, Block, Header, View,
    // headers
    MagicNumber, ChrEncodingTable, CountArrayHeader, SuffixArrayHeader, BwmHeader,
    // views
    CountArrayView, SuffixArrayView, BwmView,
};

pub struct FmIndexBuilder<P: Position, B: Block> {
    magic_number: MagicNumber,
    chr_encoding_table: ChrEncodingTable,
    count_array_header: CountArrayHeader,
    suffix_array_header: SuffixArrayHeader,
    bwm_header: BwmHeader,
    _phantom: PhantomData<(P, B)>,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    /// Blob size is not accurate.
    #[error("Blob size is not accurate. Expected: {0}, Actual: {1}")]
    InvalidBlobSize(usize, usize),
    /// Block can index up to {0} characters, but input is {1}.
    #[error("Block can index up to {0} characters, but input is {1}. Use a larger block or a smaller character set.")]
    ChrCountOver(u32, u32),
    /// Invalid lookup table k-mer size
    #[error("Lookup table kmer size must be a positive integer")]
    InvalidLookupTableKmerSize,
    /// Invalid suffix array sampling ratio
    #[error("Suffix array sampling ratio must be a positive integer")]
    InvalidSuffixArraySamplingRatio,
    /// Not aligned blob
    #[error("Blob is needed to be aligned with {0}. The offset is {1}.")]
    NotAlignedBlob(usize, usize),
}

impl<P: Position, B: Block> FmIndexBuilder<P, B> {
    pub fn init<T: AsRef<[u8]>>(
        text_len: usize,
        characters_by_index: &[T],
        // Compression options
        suffix_array_sampling_ratio: u32,
        lookup_table_kmer_size: u32,
    ) -> Result<Self, BuildError> {
        // Validate options
        if suffix_array_sampling_ratio == 0 {
            return Err(BuildError::InvalidSuffixArraySamplingRatio);
        }
        if lookup_table_kmer_size == 0 {
            return Err(BuildError::InvalidLookupTableKmerSize);
        }
        
        let chr_count = characters_by_index.len() as u32;
        let chr_encoding_table = ChrEncodingTable::new(
            characters_by_index,
        );
        if chr_count > B::MAX_CHR {
            return Err(BuildError::ChrCountOver(B::MAX_CHR, chr_count));
        }

        // Generate headers
        let count_array_header = CountArrayHeader::new(
            chr_count,
            lookup_table_kmer_size,
        );
        let suffix_array_header = SuffixArrayHeader::new(
            text_len as u64,
            suffix_array_sampling_ratio,
        );
        let bwm_header = BwmHeader::new::<P, B>(
            text_len as u64,
            chr_count + 1,
        );

        Ok(Self {
            magic_number: MagicNumber::new(),
            chr_encoding_table,
            count_array_header,
            suffix_array_header,
            bwm_header,
            _phantom: PhantomData,
        })
    }

    /// Calculate the total size of the blob
    pub fn blob_aligned_size(&self) -> usize {
        self.header_aligned_size() + self.body_aligned_size()
    }
    // 실제 헤더 사이즈
    fn header_aligned_size(&self) -> usize {
        self.magic_number.aligned_size::<B>()
        + self.chr_encoding_table.aligned_size::<B>()
        + self.count_array_header.aligned_size::<B>()
        + self.suffix_array_header.aligned_size::<B>()
        + self.bwm_header.aligned_size::<B>()
    }
    fn body_aligned_size(&self) -> usize {
        CountArrayView::<P>::aligned_body_size::<B>(&self.count_array_header)
        + SuffixArrayView::<P>::aligned_body_size::<B>(&self.suffix_array_header) 
        + BwmView::<P, B>::aligned_body_size::<B>(&self.bwm_header)
    }

    pub fn build<'a>(
        &self,
        mut text: Vec<u8>,
        blob: &'a mut [u8],
    ) -> Result<(), BuildError> {
        // FIXME: Check alignment
        let blob_size = self.blob_aligned_size();
        let blob_size_actual = blob.len();
        if blob_size != blob_size_actual {
            return Err(BuildError::InvalidBlobSize(blob_size, blob_size_actual));
        }

        // 1) Write headers
        let mut header_start_index = 0;
        // Magic number
        let mut header_end_index = self.magic_number.aligned_size::<B>();
        self.magic_number.write_to_blob(&mut blob[header_start_index..header_end_index]);
        // Chr encoding table
        header_start_index = header_end_index;
        header_end_index += self.chr_encoding_table.aligned_size::<B>();
        self.chr_encoding_table.write_to_blob(&mut blob[header_start_index..header_end_index]);
        // Count array header
        header_start_index = header_end_index;
        header_end_index += self.count_array_header.aligned_size::<B>();
        self.count_array_header.write_to_blob(&mut blob[header_start_index..header_end_index]);
        // Suffix array header
        header_start_index = header_end_index;
        header_end_index += self.suffix_array_header.aligned_size::<B>();
        self.suffix_array_header.write_to_blob(&mut blob[header_start_index..header_end_index]);
        // BWM header
        header_start_index = header_end_index;
        header_end_index += self.bwm_header.aligned_size::<B>();
        self.bwm_header.write_to_blob(&mut blob[header_start_index..header_end_index]);

        // 2) Build & write bodies
        let mut body_start_index = header_end_index;
        let mut body_end_index = body_start_index + CountArrayView::<P>::aligned_body_size::<B>(&self.count_array_header);
        // Count array
        //  - encode text with encoding table
        //  - during encoding, count the number of each character & kmer
        self.count_array_header.count_and_encode_text::<P, B>(
            &mut text,
            &self.chr_encoding_table,
            &mut blob[body_start_index..body_end_index],
        );
        // Suffix array
        //  - burrow-wheeler transform
        //  - get sentinel character index
        body_start_index = body_end_index;
        body_end_index = body_start_index + SuffixArrayView::<P>::aligned_body_size::<B>(&self.suffix_array_header);

        let sentinel_chr_index = self.suffix_array_header.write_to_blob_and_get_sentinel_chr_index::<P>(
            &mut text,
            &mut blob[body_start_index..body_end_index],
        );
        // BWM
        body_start_index = body_end_index;
        body_end_index = body_start_index + BwmView::<P, B>::aligned_body_size::<B>(&self.bwm_header);
        self.bwm_header.encode_bwm_body::<P, B>(
            text,
            sentinel_chr_index, 
            &mut blob[body_start_index..body_end_index],
        );

        Ok(())
    }
}
