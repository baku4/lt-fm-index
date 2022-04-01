use crate::*;
use crate::tests::random_text::*;

use std::io::Cursor;

#[test]
fn test_all_types_of_structures_are_serializable() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_64 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_64, loaded_lt_fm_index_64);
            
            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_128 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_128, loaded_lt_fm_index_128);
        }
        // NN
        {
            let text = rand_text_of_nn();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_64 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_64, loaded_lt_fm_index_64);

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_128 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_128, loaded_lt_fm_index_128);
        }
        // AO
        {
            let text = rand_text_of_ao();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_64 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_64, loaded_lt_fm_index_64);

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_128 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_128, loaded_lt_fm_index_128);
        }
        // AN
        {
            let text = rand_text_of_an();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_64 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_64, loaded_lt_fm_index_64);

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            let loaded_lt_fm_index_128 = LtFmIndex::load_from(Cursor::new(buffer)).unwrap();

            assert_eq!(lt_fm_index_128, loaded_lt_fm_index_128);
        }
    }
}


#[test]
fn test_all_types_of_structures_are_aware_collect_size() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();

            assert_eq!(buffer.len(), lt_fm_index_64.size_of());
            
            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();

            assert_eq!(buffer.len(), lt_fm_index_128.size_of());
        }
        // NN
        {
            let text = rand_text_of_nn();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_64.size_of());

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_128.size_of());
        }
        // AO
        {
            let text = rand_text_of_ao();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_64.size_of());

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_128.size_of());
        }
        // AN
        {
            let text = rand_text_of_an();

            // 64
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_64.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_64.size_of());

            // 128
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let mut buffer = Vec::new();
            lt_fm_index_128.save_to(&mut buffer).unwrap();
            
            assert_eq!(buffer.len(), lt_fm_index_128.size_of());
        }
    }
}
