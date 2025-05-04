# LtFmIndex
[![CI](https://github.com/baku4/lt-fm-index/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/baku4/lt-fm-index/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/lt-fm-index.svg)](https://crates.io/crates/lt-fm-index)

`LtFmIndex` is a Rust library for building and using a FM-index that contains a lookup table of the first *k-mer* of a pattern. This index can be used to (1) count the number of occurrences and (2) locate the positions of a pattern in an indexed text.

## Usage
### Add to dependency
To use this library, add `lt_fm_index` to your `Cargo.toml`:
```toml
[dependencies]
lt_fm_index = "0.7.0"
```

### Example code
```rust
use lt_fm_index::LtFmIndex;
use lt_fm_index::blocks::Block2; // `Block2` can index 3 types of characters.

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
```

### Features
- `fastbwt`: This feature can accelerate the indexing, but needs `cmake` to build `libdivsufsort` and cannot be built as WASM.
- `async-io`: This feature enables asynchronous I/O operations using Tokio for saving and loading the index. It adds support for async methods like `async_save_to` and `async_load_from` which can be used in asynchronous contexts.

## Repository
[https://github.com/baku4/lt-fm-index](https://github.com/baku4/lt-fm-index)

## API Doc
[https://docs.rs/lt-fm-index/](https://docs.rs/lt-fm-index/)

## Reference
- Ferragina, P., et al. (2004). An Alphabet-Friendly FM-Index, Springer Berlin Heidelberg: 150-160.
- Anderson, T. and T. J. Wheeler (2021). An optimized FM-index library for nucleotide and amino acid search, Cold Spring Harbor Laboratory.
- Wang, Y., X. Li, D. Zang, G. Tan and N. Sun (2018). Accelerating FM-index Search for Genomic Data Processing, ACM.
- Yuta Mori. [`libdivsufsort`](https://github.com/y-256/libdivsufsort)