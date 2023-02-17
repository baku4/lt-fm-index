#[test]
#[allow(unused_variables)]
fn example() {
    use crate::LtFmIndex;
    use crate::blocks::Block2;

    // (1) Define characters to use
    let characters_by_index: &[&[u8]] = &[
        &[b'A', b'a'], // 'A' and 'a' are treated as the same
        &[b'C', b'c'], // 'C' = 'c'
        &[b'G', b'g'], // 'G' = 'g'
    ];
    // Or
    let characters_by_index: &[&[u8]] = &[
        b"Aa", b"Cc", b"Gg"
    ];

    // (2) Build index
    let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
    let lt_fm_index= LtFmIndex::<u32, Block2<u128>>::build(
        text,
        characters_by_index,
        2,
        4,
    ).unwrap();

    // (3) Match with pattern
    let pattern = b"TA";
    //   - count
    let count = lt_fm_index.count(pattern);
    assert_eq!(count, 2);
    //   - locate
    let locations = lt_fm_index.locate(pattern);
    println!("{:?}", locations);
    assert_eq!(locations, vec![5,18]);
}
