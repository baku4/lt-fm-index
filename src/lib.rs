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
// Data structures
mod structures;
pub use structures::{
    LtFmIndex,
    TextType,
    BwtBlockSize,
};
// Builder
mod builder;
pub use builder::{
    LtFmIndexBuilder,
};
/// Errors
pub mod errors;

// ## Supplement
#[doc(hidden)]
#[allow(dead_code)]
pub mod tests;
