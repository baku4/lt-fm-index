pub type SuffixArray = Vec<u64>;

// Accumualte count array
#[inline]
pub fn accumulate_count_array(count_array: &mut [u64]) {
    let mut accumed_count: u64 = 0;
    count_array.iter_mut().for_each(|count| {
        accumed_count += *count;
        *count = accumed_count;
    });
}

// Compress suffix array
#[inline]
pub fn compress_suffix_array(suffix_array: Vec<i64>, sampling_ratio: u64) -> SuffixArray {
    if sampling_ratio == 1 {
        suffix_array.into_iter().map(|x| x as u64).collect()
    } else {
        suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
    }
}