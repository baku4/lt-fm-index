use crate::core::Position;

mod burrow_wheeler_transform;
use burrow_wheeler_transform::get_compressed_suffix_array_and_pidx_while_bwt;
use num_integer::div_rem;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArrayHeader {
    // Given
    pub sampling_ratio: u32,
    _padding: u32,
    // Derivatives
    pub suffix_array_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArrayView<'a, P: Position> {
    suffix_array: &'a [P],
}

// ================================================
// Build
// ================================================
impl SuffixArrayHeader {
    pub fn new(
        sampling_ratio: u32,
        text_len: u64,
    ) -> Self {
        let (q, r) = div_rem(text_len, sampling_ratio as u64);
        //FIXME: array len calculation is correct?
        let suffix_array_len = if r  == 0 {
            q
        } else {
            q + 1
        };

        Self { sampling_ratio, suffix_array_len, _padding: 0 }
    }
    pub fn write_to_blob_and_get_pidx<P: Position>(
        &self,
        text: &mut Vec<u8>,
        blob: &mut [P],
    ) -> P {
        let (compressed_suffix_array, pidx) = get_compressed_suffix_array_and_pidx_while_bwt::<P>(
            text,
            P::from_u32(self.sampling_ratio),
        );

        // Write compressed_suffix_array into blob using zerocopy
        assert_eq!(blob.len(), compressed_suffix_array.len(), "Blob and compressed_suffix_array must have the same length");
        blob.copy_from_slice(&compressed_suffix_array);

        pidx
    }
}

// ================================================
// Locate
// ================================================
impl<'a, P: Position> SuffixArrayView<'a, P> {
    pub fn get_location_of(
        &self,
        position: P,
        sampling_ratio: P,
    ) -> P {
        self.suffix_array[(position / sampling_ratio).as_usize()]
    }
}
