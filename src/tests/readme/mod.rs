#[test]
#[allow(unused_variables)]
fn example() {

use crate::LtFmIndex;
use crate::blocks::Block2; // Block2 can index 3 types of characters.

// (1) Define characters to use
let characters_by_index: &[&[u8]] = &[
    &[b'A', b'a'], // 'A' and 'a' are treated as the same
    &[b'C', b'c'], // 'C' and 'c' are treated as the same
    &[b'G', b'g'], // 'G' and 'g' are treated as the same
];
// Alternatively, you can use this simpler syntax:
let characters_by_index: &[&[u8]] = &[
    b"Aa", b"Cc", b"Gg"
];

// (2) Build index
let text = b"CTCCGTACACCTGTTTCGTATCGGAXXYYZZ".to_vec();
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
let mut locations = lt_fm_index.locate(pattern);
locations.sort();  // The locations may not be in order.
assert_eq!(locations, vec![5,18]);
// All unindexed characters are treated as the same character.
// In the text, X, Y, and Z can match any other unindexed character
let mut locations = lt_fm_index.locate(b"UNDEF");
locations.sort();
// Using the b"XXXXX", b"YYYYY", or b"!@#$%" gives the same result.
assert_eq!(locations, vec![25,26]);

// (4) Save and load
let mut buffer = Vec::new();
lt_fm_index.save_to(&mut buffer).unwrap();
let loaded = LtFmIndex::load_from(&buffer[..]).unwrap();
assert_eq!(lt_fm_index, loaded);

}
