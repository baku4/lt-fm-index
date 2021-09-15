// use crate::structure::{LtFmIndex, CountArrayInterface, BwtInterface};
// use crate::structure::count_array::CountArray;
// use crate::structure::bwt::{Bwt, BwtBlockInterface};

// use libdivsufsort_rs::{divsufsort64, bw_transform64};

// type Text = Vec<u8>;

// fn build(
//     mut text: Text,
//     sa_sampling_ratio: u64,
//     kmer_size: Option<usize>,
//     bitcount_size: BitCountSize,
//     chr_count: usize,
// ) {
//     let text_len = text.len() as u64;
//     // (1) count array
//     match kmer_size {
//         Some(kmer) => {
//             let mut counts: Vec<u64> = vec![0; chr_count];

//             let klt_length: usize = 5usize.pow(kmer as u32);
//             let mut kmer_lookup_table: Vec<u64> = vec![0; klt_length];
//             let mut klt_index: usize = 0;
//             // Pre-cal index
//             let index_for_each_char: [usize; 4] = {
//                 let i = klt_length/5;
//                 [1*i, 2*i, 3*i, 4*i]
//             };
//             // Use each char
//             text.iter_mut().rev().for_each(|chr| {
//                 match *chr {
//                     A_UTF8 => {
//                         counts[1] += 1;
//                         klt_index /= 5;
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     C_UTF8 => {
//                         counts[2] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[0];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     G_UTF8 => {
//                         counts[3] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[1];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     T_UTF8 => {
//                         counts[4] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[2];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     _ => {
//                         counts[5] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[3];
//                         kmer_lookup_table[klt_index] += 1;
//                         *chr = N_UTF8;
//                     },
//                 }
//             });
//             // accumulate array
//             accumulate_count_array(&mut counts);
//             accumulate_count_array(&mut kmer_lookup_table);
//             (counts, Some((kmer, kmer_lookup_table)))
//         },
//         None => {
//             let mut count_array: CountArray = [0; 6];
//             for chr in text {
//                 match *chr {
//                     A_UTF8 => count_array[1] += 1,
//                     C_UTF8 => count_array[2] += 1,
//                     G_UTF8 => count_array[3] += 1,
//                     T_UTF8 => count_array[4] += 1,
//                     _ => {
//                         count_array[5] += 1;
//                         *chr = N_UTF8;
//                     },
//                 }
//             }
//             accumulate_count_array(&mut count_array);
//             (count_array, None)
//         }
//     }
//     let count_array
// }

// fn count_array_build(
//     text: &mut Text,
//     kmer_size: Option<usize>,
//     chr_count: usize,
// ) {
//     match kmer_size {
//         Some(k) => {
//             let mut counts: Vec<u64> = vec![0; chr_count];

//             let multiplier: Vec<usize> = (0..k as u32).rev().map(|pos| {chr_count.pow(pos)}).collect();
//             let offset = multiplier[0];
//             let klt_length: usize = offset * (chr_count-1);

//             let mut kmer_lookup_table: Vec<u64> = vec![0; klt_length];
//             let mut klt_index: usize = 0;
            
//             text.iter_mut().rev().for_each(|chr| {
//                 match *chr {
//                     A_UTF8 => {
//                         count_array[1] += 1;
//                         klt_index /= 5;
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     C_UTF8 => {
//                         count_array[2] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[0];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     G_UTF8 => {
//                         count_array[3] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[1];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     T_UTF8 => {
//                         count_array[4] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[2];
//                         kmer_lookup_table[klt_index] += 1;
//                     },
//                     _ => {
//                         count_array[5] += 1;
//                         klt_index /= 5;
//                         klt_index += index_for_each_char[3];
//                         kmer_lookup_table[klt_index] += 1;
//                         *chr = N_UTF8;
//                     },
//                 }
//             });
// }

// enum BitCountSize {
//     Bit8,
//     Bit16,
// }