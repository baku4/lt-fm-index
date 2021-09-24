#[test]
fn example_1() {
    use crate::{FmIndex, LtFmIndexConfig};

    // (1) Define configuration for lt-fm-index
    let config = LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(4).unwrap()
        .change_sampling_ratio(4).unwrap()
        .change_bwt_interval_to_128();

    // (2) Generate fm-index with text
    let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
    let lt_fm_index = config.generate(text).unwrap(); // text is consumed

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
fn example_2() {
    use crate::{LtFmIndexConfig, LtFmIndexAll, IO};

    // (1) Generate `FmIndex`
    let config = LtFmIndexConfig::for_nucleotide();
    let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
    let lt_fm_index = config.generate(text).unwrap(); // text is consumed

    // (2) Write fm-index to buffer (or file path)
    let mut buffer = Vec::new();
    lt_fm_index.write_to(&mut buffer).unwrap();

    // (3) Read fm-index from buffer (or file path)
    let lt_fm_index_buf = LtFmIndexAll::read_from(&buffer[..]).unwrap();

    assert_eq!(lt_fm_index, lt_fm_index_buf);
}
