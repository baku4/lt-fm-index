use crate::new_algorithm::{Header, View};

use super::{
    FmIndex, Position, Block,
    // headers
    MagicNumber, ChrEncodingTable, CountArrayHeader, SuffixArrayHeader, BwmHeader,
    // views
    CountArrayView, SuffixArrayView, BwmView,
};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// Error while loading Fm-index headers.
    #[error("Error while loading Fm-index headers. Not a valid FM-index blob.")]
    InvalidHeaders,
    /// Blob size is not accurate.
    #[error("Invalid blob size. Expected: {0}, Actual: {1}")]
    InvalidBlobSize(usize, usize),
}


impl<'a, P: Position, B: Block> FmIndex<'a, P, B> {
    /// Load fm-index from blob
    pub fn load(blob: &'a [u8]) -> Result<Self, LoadError> {
        // Load headers
        let (magic_number, remaining_bytes) = MagicNumber::read_from_blob::<B>(blob);
        let (chr_encoding_table, remaining_bytes) = ChrEncodingTable::read_from_blob::<B>(remaining_bytes);
        let (count_array_header, remaining_bytes) = CountArrayHeader::read_from_blob::<B>(remaining_bytes);
        let (suffix_array_header, remaining_bytes) = SuffixArrayHeader::read_from_blob::<B>(remaining_bytes);
        let (bwm_header, body_blob) = BwmHeader::read_from_blob::<B>(remaining_bytes);

        // check body size
        let actual_body_size = body_blob.len();
        let expected_body_size = {
            CountArrayView::<P>::aligned_body_size::<B>(&count_array_header)
            + SuffixArrayView::<P>::aligned_body_size::<B>(&suffix_array_header)
            + BwmView::<P, B>::aligned_body_size::<B>(&bwm_header)
        };
        if actual_body_size != expected_body_size {
            let header_size = {
                magic_number.aligned_size::<B>()
                + chr_encoding_table.aligned_size::<B>()
                + count_array_header.aligned_size::<B>()
                + suffix_array_header.aligned_size::<B>()
                + bwm_header.aligned_size::<B>()
            };
            return Err(LoadError::InvalidBlobSize(
                header_size + expected_body_size,
                header_size + actual_body_size,
            ));
        }

        // Get views
        //  - Count array
        let mut body_start_index = 0;
        let mut body_end_index = CountArrayView::<P>::aligned_body_size::<B>(&count_array_header);
        let count_array_view = CountArrayView::<P>::load_from_body::<B>(&count_array_header, &body_blob[body_start_index..body_end_index]);
        //  - Suffix array
        body_start_index = body_end_index;
        body_end_index += SuffixArrayView::<P>::aligned_body_size::<B>(&suffix_array_header);
        let suffix_array_view = SuffixArrayView::<P>::load_from_body::<B>(&suffix_array_header, &body_blob[body_start_index..body_end_index]);
        //  - BWM
        body_start_index = body_end_index;
        body_end_index += BwmView::<P, B>::aligned_body_size::<B>(&bwm_header);
        let bwm_view = BwmView::<P, B>::load_from_body::<B>(&bwm_header, &body_blob[body_start_index..body_end_index]);

        Ok(Self {
            magic_number,
            chr_encoding_table,
            count_array_header,
            suffix_array_header,
            bwm_header,
            count_array_view,
            suffix_array_view,
            bwm_view,
        })
    }
}