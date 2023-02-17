use crate::core::Position;

#[allow(dead_code)]
mod burrow_wheeler_transform;
use burrow_wheeler_transform::get_compressed_suffix_array_and_pidx_while_bwt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArray<P: Position> {
    sampling_ratio: P,
    array: Vec<P>,
}

impl<P: Position> SuffixArray<P> {
    // Build
    pub fn new_while_bwt(text: &mut Vec<u8>, sasr: P) -> (Self, P) {
        let (compressed_suffix_array, pidx) = get_compressed_suffix_array_and_pidx_while_bwt(text, sasr);

        let suffix_array = Self {
            sampling_ratio: sasr,
            array: compressed_suffix_array,
        };
        (suffix_array, pidx)
    }

    // Locate
    pub fn sampling_ratio(&self) -> P {
        self.sampling_ratio
    }
    pub fn get_location_of(&self, position: P) -> P {
        self.array[(position / self.sampling_ratio).as_usize()]
    }
}

mod serialize;

#[cfg(test)]
mod tests {
    fn compress_suffix_array(suffix_array: Vec<u32>, sampling_ratio: u32) -> Vec<u32> {
        if sampling_ratio == 1 {
            suffix_array.into_iter().map(|x| x as u32).collect()
        } else {
            suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x ).collect()
        }
    }

    #[test]
    fn test_compress_suffix_array() {
        let raw_suffix_array: Vec<u32> = (0..30).collect();
        let sampling_ratio: u32 = 5;
        let sa = compress_suffix_array(raw_suffix_array, sampling_ratio);
        assert_eq!(sa, vec![0, 5, 10, 15, 20, 25]);
    }
}
