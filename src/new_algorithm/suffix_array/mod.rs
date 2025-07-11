use crate::core::Position;

mod burrow_wheeler_transform;
use burrow_wheeler_transform::get_compressed_suffix_array_and_pidx_while_bwt;
use num_integer::div_rem;
use zerocopy::IntoBytes;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
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
    // From header
    sampling_ratio: P,
    // From blob
    suffix_array: &'a [P],
}

// ================================================
// Build
// ================================================
impl SuffixArrayHeader {
    pub fn new(
        text_len: u64,
        sampling_ratio: u32,
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
    pub fn calculate_body_size<P: Position>(
        &self,
    ) -> usize {
        self.suffix_array_len as usize * std::mem::size_of::<P>()
    }
    pub fn write_to_blob_and_get_sentinel_chr_index<P: Position>(
        &self,
        text: &mut Vec<u8>,
        blob: &mut [u8],
    ) -> P {
        let (compressed_suffix_array, pidx) = get_compressed_suffix_array_and_pidx_while_bwt::<P>(
            text,
            P::from_u32(self.sampling_ratio),
        );

        blob.copy_from_slice(compressed_suffix_array.as_bytes());

        pidx
    }
}

// ================================================
// Load
// ================================================
impl SuffixArrayHeader {
    pub fn load<'a, P: Position>(&self, body_blob: &'a [u8]) -> SuffixArrayView<'a, P> {
        let sampling_ratio = P::from_u32(self.sampling_ratio);
        let suffix_array: &[P] = zerocopy::FromBytes::ref_from_bytes(body_blob).unwrap();

        SuffixArrayView {
            sampling_ratio,
            suffix_array,
        }
    }
}

// ================================================
// Locate
// ================================================
impl<'a, P: Position> SuffixArrayView<'a, P> {
    pub fn get_location_of(
        &self,
        position: P,
    ) -> P {
        self.suffix_array[(position / self.sampling_ratio).as_usize()]
    }
}
