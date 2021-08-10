# LT FM-Index
`lt-fm-index` is library for locate and count nucleotide sequence (ATGC) string.  
`lt-fm-index` using k-mer lookup table (As you noticed, LT stands for lookup table).

## Description
- Fm-index is a data structure used for pattern matching.
- K-mer lookup table(KLT) is precalculated count table containing all kmer occurrences.
- With KLT, you can find the first k-mer pattern at once.
- Supports two types of text.
  - `FmIndexOn` supports a text with only genetic nucleotide sequence (ACGT).
  - `FmIndexNn` supports a text containing non-nucleotide sequence.
    - `FmIndexNn` treats all non-nucleotide as the same character.
- **CAVEAT!** This `crate` is not stable. Functions can be changed without notice.
## Features
- Fm-index using KLT with specified k-mer size.
- Suffix array compression with sampling ratio.
- BWT and suffix array are generated using `libdivsufsort` library.
- BWT(burrow wheeler transformed) string and occurrence array (OA) are aligned in one block of 64 strings.
- There are two main functions.
    - count: Count the number of patterns in the text
    - locate: Locate pattern index in text (KLT can be specified to enable or not)

## Examples
### 1. Use `FmIndex` to locate pattern.
```rust
use lt_fm_index::FmIndexConfig;

// (1) Define configuration for fm-index
let fmi_config = FmIndexConfig::new()
	.set_kmer_lookup_table(8)
	.set_suffix_array_sampling_ratio(4)
	.contain_non_nucleotide(); // Default is `contain only nucleotide`

// (2) Generate fm-index with text
let text = b"CTCCGTACACCTGTTTCGTATCGGANNN".to_vec();
let fm_index = fmi_config.generate_fmindex(text); // text is consumed

// (3) Match with pattern
let pattern = b"TA".to_vec();
//   - count
let count = fm_index.count(&pattern);
assert_eq!(count, 2);
//   - locate without k-mer lookup table
let locations = fm_index.locate_wo_klt(&pattern);
assert_eq!(locations, vec![5,18]);
//   - locate with k-mer lookup table
let locations = fm_index.locate_w_klt(&pattern);
assert_eq!(locations, vec![5,18]);
```
### 2. Write and read `FmIndex`
```rust
use lt_fm_index::{FmIndexConfig, FmIndex};

// (1) Generate `FmIndex`
let fmi_config = FmIndexConfig::new()
    .set_kmer_lookup_table(8)
    .set_suffix_array_sampling_ratio(4);
let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
let fm_index_pre = fmi_config.generate_fmindex(text); // text is consumed

// (2) Write fm-index to buffer (or file path)
let mut buffer = Vec::new();
fm_index_pre.write_index_to(&mut buffer).unwrap();

// (3) Read fm-index from buffer (or file path)
let fm_index_pro = FmIndex::read_index_from(&buffer[..]).unwrap();

assert_eq!(fm_index_pre, fm_index_pro);
```
## Future works
- Support *SIMD* for BWT block compression.
- Length of texts can be `32bit` integer
## Repository
[https://github.com/baku4/lt-fm-index](https://github.com/baku4/lt-fm-index)
## Doc
[https://docs.rs/lt-fm-index/](https://docs.rs/lt-fm-index/)
## Reference
- Ferragina, P., et al. (2004). An Alphabet-Friendly FM-Index, Springer Berlin Heidelberg: 150-160.
- Anderson, T. and T. J. Wheeler (2021). An optimized FM-index library for nucleotide and amino acid search, Cold Spring Harbor Laboratory.
- Wang, Y., X. Li, D. Zang, G. Tan and N. Sun (2018). Accelerating FM-index Search for Genomic Data Processing, ACM.
- Yuta Mori. [`libdivsufsort`](https://github.com/y-256/libdivsufsort)