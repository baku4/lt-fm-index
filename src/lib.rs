/*! # LT FM-Index
`lt-fm-index` is library for locate and count nucleotide and amino acid sequence string.  
`lt-fm-index` use lookup table (LT) in count table

**CAVEAT!** This `crate` is not stable. Functions can be changed without notification.
## Description
- Fm-index is a data structure used for pattern matching.
- `LT` is precalculated count table containing all kmer occurrences.
- With `LT`, you can find the first k-mer pattern at once.
## Features
- `LtFmIndex` is generated from `Text`
- `LtFmIndex` have two functions for `Pattern`
    - count: Count the number of times the `Pattern` appears in `Text`.
    - locate: Locate the start index in which the `Pattern` appears in `Text`.
- Supports **four** types of text.
    - `NucleotideOnly` supports a text with only genetic nucleotide sequence (ACGT).
    - `NucleotideWithNoise` supports a text containing non-nucleotide sequence.
    - `AminoacidOnly` supports a text with only amino acid sequence.
    - `AminoacidWithNoise` supports a text containing non-amino acid sequence.
- The last character of each text type is treated as a wildcard.
    - The last characters of each text type are *T*, *_*, *Y* and *_*.
    - Wildcard is assigned to all non-supported characters.
    - For example, in `NucleotideOnly`, pattern of *ACGTXYZ* can be matched with *ACGTTTT*. Because *X*, *Y* and *Z* are not in *ACG* (nucleotide except *T*). And `lt-fm-index` generated with text of *ACGTXYZ* indexes the text as *ACGTTTT*.
- BWT is stored with rank count tables in every 64 or 128 intervals.
## Examples
### 1. Use `LtFmIndex` to count and locate pattern.
```rust
use lt_fm_index::{FmIndex, LtFmIndexConfig};

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
```
### 2. Write and read `LtFmIndex`
```rust
use lt_fm_index::{LtFmIndexConfig, LtFmIndexAll, IO};

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

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub mod deprecated;
#[doc(hidden)]
pub mod tests;
pub mod unarchived;

// Core types and requirements
mod core;
// Basic data structures
mod structure;
// Implementations by text type
mod text_type;
