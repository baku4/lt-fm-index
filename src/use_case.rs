use crate::proto::{CountArray, CountArrayProto};
use crate::proto::{Bwt, BwtProto, BwtBlock};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

mod nc_with_noise;


use nc_with_noise::{CountArrayNN, Bwt64NN};

fn test() {
    use crate::structure::LtFmIndex;

    let lt_fm_index:LtFmIndex<CountArrayNN, Bwt64NN> = LtFmIndex::new(
        b"AT".to_vec(), 1, Some(3),
    );
}