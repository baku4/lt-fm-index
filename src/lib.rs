mod core;
pub use crate::core::Position;
mod algorithm;
pub use algorithm::{
    LtFmIndex,
    Block,
    blocks,
};

mod wrapper;

#[cfg(test)]
mod tests;

#[test]
fn example() {
    use crate::LtFmIndex;
    use blocks::Block3;

    let characters_by_index: &[&[u8]] = &[
        &[b'A', b'a'],
        &[b'C', b'c'],
        &[b'G', b'g'],
    ];
    let characters_by_index: &[&[u8]] = &[
        b"Aa", b"Cc", b"Gg"
    ];

    // let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
    let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
    let lt_fm_index= LtFmIndex::<u32, Block3<u128>>::new(
        text,
        characters_by_index,
        2,
        4,
    );

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
