/*!# LtFmIndex
[![CI](https://github.com/baku4/lt-fm-index/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/baku4/lt-fm-index/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/lt-fm-index.svg)](https://crates.io/crates/lt-fm-index)

`lt-fm-index` is a library to (1) locate or (2) count the pattern in the large text of nucleotide and amino acid sequences.
## Description
- *FmIndex* is a data structure for exact pattern matching.
- `LtFmIndex` is *FmIndex* using lookup table, the precalculated count of *k-mer* occurrences.
  - The lookup table can locate the first *k-mer* of pattern at once.
## Features
- `LtFmIndex` is built from `Text` (`Vec<u8>`).
- `LtFmIndex` have two functions.
    1. `count`: Count the number of times the `Pattern` (`&[u8]`) appears in the `Text`.
    2. `locate`: Locate the start index in which the `Pattern` appears in the `Text`.
- **Four** types of `Text` are supported.
    - `NucleotideOnly`: consists of {ACG*}
    - `NucleotideWithNoise`: consists of {ACGT*}
    - `AminoacidOnly`: consists of {ACDEFGHIKLMNPQRSTVW*}
    - `AminoacidWithNoise`: consists of {ACDEFGHIKLMNPQRSTVWY*}
- The `*` of each type is treated as a *wildcard* that can be matched with any characters.
    - For example,
        - If the TextType is `NucleotideOnly`, `LtFmIndex` stores the text of *ACGTXYZ* as <i>ACG****</i>.
        - If the TextType is `NucleotideWithNoise`, `LtFmIndex` stores the same text (*ACGTXYZ*) as <i>ACGT***</i>
        - If the indexed text is <i>ACGT***</i>, the patterns of *ACGTXXX*, *ACGT@@@*, and *ACGTX@#* give the same result.
- Using `fastbwt` feature can accelerate the indexing, but needs `cmake` to build `libdivsufsort` and cannot be built as WASM.
## Examples
### 1. Use `LtFmIndex` to count and locate a pattern.
```rust
use lt_fm_index::LtFmIndexBuilder;

// (1) Define builder for lt-fm-index
let builder = LtFmIndexBuilder::new()
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
```
### 2. Save and load `LtFmIndex`
```rust
use lt_fm_index::{LtFmIndex, LtFmIndexBuilder};

// (1) Generate lt-fm-index
let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
let lt_fm_index_to_save = LtFmIndexBuilder::new().build(text).unwrap();

// (2) Save lt-fm-index to buffer
let mut buffer = Vec::new();
lt_fm_index_to_save.save_to(&mut buffer).unwrap();

// (3) Load lt-fm-index from buffer
let lt_fm_index_loaded = LtFmIndex::load_from(&buffer[..]).unwrap();

assert_eq!(lt_fm_index_to_save, lt_fm_index_loaded);
```
## Repository
[https://github.com/baku4/lt-fm-index](https://github.com/baku4/lt-fm-index)
## Doc
[https://docs.rs/lt-fm-index/](https://docs.rs/lt-fm-index/)
## Reference
- Ferragina, P., et al. (2004). An Alphabet-Friendly FM-Index, Springer Berlin Heidelberg: 150-160.
- Anderson, T. and T. J. Wheeler (2021). An optimized FM-index library for nucleotide and amino acid search, Cold Spring Harbor Laboratory.
- Wang, Y., X. Li, D. Zang, G. Tan and N. Sun (2018). Accelerating FM-index Search for Genomic Data Processing, ACM.
- Yuta Mori. [`libdivsufsort`](https://github.com/y-256/libdivsufsort)
*/

// Core types and requirements
mod core;
// Structures
mod structures;
pub use structures::{
    LtFmIndex,
    TextEncoder,
    text_encoders,
};
// Builder
mod builder;

#[test]
fn example_1() {
    use crate::LtFmIndex;
    use crate::text_encoders::C3B64;

    let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();

    let text_encoder = C3B64::new([
        vec![b'A', b'a'],
        vec![b'C', b'c'],
        vec![b'G', b'g'],
    ]);
    let lt_fm_index = LtFmIndex::new(
        text,
        &text_encoder,
        2,
        4,
    );

    let pattern = b"TA".to_vec();
    //   - count
    let count = lt_fm_index.count(&pattern);
    assert_eq!(count, 2);
    //   - locate
    let locations = lt_fm_index.locate(&pattern);
    assert_eq!(locations, vec![5,18]);
}

// Data structures
mod structures_dep;
use structures_dep::{
    LtFmIndexDep,
    TextTypeDep,
    BwtBlockSizeDep,
};
// Builder
mod builder_dep;
use builder_dep::{
    LtFmIndexBuilderDep,
};
/// Errors
pub mod errors;

// ## Supplement
#[doc(hidden)]
#[allow(dead_code)]
pub mod tests;
