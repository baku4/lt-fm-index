#[test]
fn example_in_doc_1() {
    use crate::LtFmIndexBuilderDep;

    // (1) Define builder for lt-fm-index
    let builder = LtFmIndexBuilderDep::new()
        .text_type_is_inferred()
        .set_suffix_array_sampling_ratio(2).unwrap()
        .set_lookup_table_kmer_size(4).unwrap();

    // (2) Generate lt-fm-index with text
    let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
    let lt_fm_index = builder.build(text).unwrap(); // text is consumed

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
    use crate::{LtFmIndexDep, LtFmIndexBuilderDep};

    // (1) Generate lt-fm-index
    let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
    let lt_fm_index_to_save = LtFmIndexBuilderDep::new().build(text).unwrap();

    // (2) Save lt-fm-index to buffer
    let mut buffer = Vec::new();
    lt_fm_index_to_save.save_to(&mut buffer).unwrap();

    // (3) Load lt-fm-index from buffer
    let lt_fm_index_loaded = LtFmIndexDep::load_from(&buffer[..]).unwrap();

    assert_eq!(lt_fm_index_to_save, lt_fm_index_loaded);
}
