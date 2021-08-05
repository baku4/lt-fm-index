# LT FM-Index

`lt-fm-index` is library for locate and count nucleotide sequence (ATGC) string.  
`lt-fm-index` using k-mer lookup table (As you noticed, LT stands for lookup table).
## Description
- Fm-index is a data structure used for pattern matching.
- K-mer lookup table(KLT) is precalculated count table containing all kmer occurrences.
- With KLT, you can find the first k-mer pattern at once.
- Currently, only the genetic sequence (ATGC) can be used.
## Features
- Fm-index using KLT with specified k-mer size.
- Suffix array compression with sampling ratio.
- BWT and suffix array are generated using `libdivsufsort` library.
- BWT(burrow wheeler transformed) string and occurrence array (OA) are aligned in one block of 64 strings.
- Aligned BWT&OA block encodes 1-byte character in 6-bits.
- There are two main functions.
    - count: Count the number of patterns in the text
    - locate: Locate pattern index in text (KLT can be specified to enable or disable)
## Future work
- IO
- Input text can be `slice`
## Example
```rust
use lt_fm_index::{Config, FmIndex};

let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
let config = Config::new()
    .set_kmer_lookup_table(8)
    .set_suffix_array_sampling_ratio(4);
let fm_index = FmIndex::new(&config, text);
let pattern = b"TA".to_vec();

// count
let count = fm_index.count(&pattern);
assert_eq!(count, 2);

// locate without k-mer lookup table
let locations = fm_index.locate(&pattern);
assert_eq!(locations, vec![5,18]);

// locate with k-mer lookup table
let locations = fm_index.locate_with_klt(&pattern);
assert_eq!(locations, vec![5,18]);
```
## Docs
[`lt-fm-index`](https://docs.rs/lt-fm-index/)
## Reference
- Ferragina, P., et al. (2004). An Alphabet-Friendly FM-Index, Springer Berlin Heidelberg: 150-160.
- Anderson, T. and T. J. Wheeler (2021). An optimized FM-index library for nucleotide and amino acid search, Cold Spring Harbor Laboratory.
- Wang, Y., X. Li, D. Zang, G. Tan and N. Sun (2018). Accelerating FM-index Search for Genomic Data Processing, ACM.
- Yuta Mori. [`libdivsufsort`](https://github.com/y-256/libdivsufsort)