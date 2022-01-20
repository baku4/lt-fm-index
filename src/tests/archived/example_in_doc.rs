#[test]
fn example_in_doc_1() {
    use crate::LtFmIndexBuilder;

    // (1) Define builder for lt-fm-index
    let builder = LtFmIndexBuilder::new()
        .use_nucleotide_with_noise()
        .set_lookup_table_kmer_size(4).unwrap()
        .set_suffix_array_sampling_ratio(2).unwrap();

    // (2) Generate lt-fm-index with text
    let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
    let lt_fm_index = builder.build(text); // text is consumed

    // (3) Match with pattern
    let pattern = b"TA".to_vec();
    //   - count
    let count = lt_fm_index.count(&pattern);
    assert_eq!(count, 2);
    //   - locate
    let locations = lt_fm_index.locate(&pattern);
    assert_eq!(locations, vec![5,18]);
}

#[test]
fn example_in_doc_2() {
    use crate::{LtFmIndex, LtFmIndexBuilder};

    // (1) Generate lt-fm-index
    let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
    let lt_fm_index_to_save = LtFmIndexBuilder::new().build(text);

    // (2) Save lt-fm-index to buffer
    let mut buffer = Vec::new();
    lt_fm_index_to_save.save_to(&mut buffer).unwrap();

    // (3) Load lt-fm-index from buffer
    let lt_fm_index_loaded_checked = LtFmIndex::load_from(&buffer[..]).unwrap();
    //   - If you are sure that this buffer is LtFmIndex.
    let lt_fm_index_loaded_unchecked = LtFmIndex::unchecked_load_from(&buffer[..]).unwrap();

    assert_eq!(lt_fm_index_to_save, lt_fm_index_loaded_checked);
    assert_eq!(lt_fm_index_to_save, lt_fm_index_loaded_unchecked);
}
