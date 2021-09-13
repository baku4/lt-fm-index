pub trait FmIndex {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}

pub type Pattern<'a> = &'a [u8];