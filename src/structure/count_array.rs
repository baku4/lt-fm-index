use super::CountArrayInterface;

struct CountArray {
    count_array: Vec<u64>,
    kmer_lookup_table: Option<KmerLookupTable>,
}

struct KmerLookupTable {}