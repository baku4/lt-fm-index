/*! # LtFmIndex
[![CI](https://github.com/baku4/lt-fm-index/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/baku4/lt-fm-index/actions/workflows/rust.yml)
![crates.io](https://img.shields.io/crates/v/lt-fm-index.svg)

`lt-fm-index` is a library to (1) locate or (2) count the pattern in the large text of nucleotide and amino acid sequences.
## Description
- *Fm-Index* is a data structure for exact pattern matching.
- `LtFmIndex` is *Fm-Index* using lookup table, the precalculated count of *k-mer* occurrences.
  - The lookup table can locate the first *k-mer* pattern at once.
## Features
- `LtFmIndex` is made from UTF-8–encoded `Text`.
- `LtFmIndex` have two functions.
    1. `count`: Count the number of times the UTF-8–encoded `Pattern` appears in the `Text`.
    2. `locate`: Locate the start index in which the `Pattern` appears in the `Text`.
- **Four** types of `Text` are supported.
    - `NucleotideOnly`: consists of  {ACGT}
    - `NucleotideWithNoise`: consists of  {ACGT_}
    - `AminoacidOnly`: consists of {ACDEFGHIKLMNPQRSTVWY}
    - `AminoacidWithNoise`: consists of {ACDEFGHIKLMNPQRSTVWY_}
- The last character of each text type (T, _, Y, _) is treated as a *wildcard* representing all unsupported characters.
    - For example, in `NucleotideOnly`:
        - `LtFmIndex` stores the text of *ACGTXYZ* as *ACGTTTT*, transforming the unsupported characters (X, Y, Z) to wildcard (T).
        - The patterns of *ACGTXXX*, *ACGXXXX*, and *ACGXYZZ* are matched with *ACGTTTT*.
## Examples
### 1. Use `LtFmIndex` to count and locate a pattern.
```rust
use lt_fm_index::LtFmIndexBuilder;

// (1) Define builder for lt-fm-index
let builder = LtFmIndexBuilder::new()
    .use_nucleotide_with_noise()
    .set_lookup_table_kmer_size(4).unwrap()
    .set_suffix_array_sampling_ratio(2).unwrap();

// (2) Generate lt-fm-index from text
let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
let lt_fm_index = builder.build(text); // text is consumed

// (3) Match with a pattern
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
let lt_fm_index_to_save = LtFmIndexBuilder::new().build(text);

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

// # Modules

// ## Supplement
#[doc(hidden)]
#[allow(dead_code)]
pub mod tests;

// ## Main
// Core types and requirements for lt-fm-index
mod core;
// Data structure
mod structure;
// Integration of data structure
#[doc(hidden)] // Make public for benchmark, not assumed to be used by end-users.
pub mod composition;
// Encoded wrapper
mod encoded;

// # API
// Public Struct
pub use encoded::{LtFmIndex, LtFmIndexBuilder};
// Public Enum
pub use composition::{TextType, BwtCompressionSize};
// Public Type
pub use self::core::{Text, Pattern};
