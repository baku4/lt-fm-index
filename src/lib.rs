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

    // let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
    let text = b"CTCCG_ACACC_G___CG_A_CGGA".to_vec();
    let lt_fm_index= LtFmIndex::<u32, Block3<u128>>::new(
        text,
        &[
            &[b'A', b'a'],
            &[b'C', b'c'],
            &[b'G', b'g'],
            // &[b'T', b't'],
        ],
        2,
        4,
    );

    // (3) Match with pattern
    let pattern = b"TA".to_vec();
    //   - count
    let count = lt_fm_index.count(&pattern);
    assert_eq!(count, 2);
    //   - locate
    let locations = lt_fm_index.locate(&pattern);
    assert_eq!(locations, vec![5,18]);
}
