/*! # LT FM-Index
`lt-fm-index` is library for locate and count nucleotide and amino acid sequence string.  
`lt-fm-index` use lookup table (LT) in count table
## Description
- Fm-index is a data structure for exact pattern matching.
- `LT` is precalculated count table containing all kmer occurrences.
- `LT` allows you to find the first k-mer pattern at once.
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
use lt_fm_index::LtFmIndexBuilder;

// (1) Define builder for lt-fm-index
let builder = LtFmIndexBuilder::new()
    .use_nucleotide_with_noise()
    .set_lookup_table_kmer_size(4).unwrap()
    .set_suffix_array_sampling_ratio(2).unwrap();

// (2) Generate lt-fm-index with text
let text = b"CTCCGTACACCTGTTTCGTATCGGANNNN".to_vec();
let lt_fm_index = builder.build(text); // text is consumed

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
pub mod tests;
#[allow(dead_code)]
pub mod deprecated;

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
