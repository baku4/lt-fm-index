pub mod random_text;
#[cfg(test)]
pub mod other_crate;

pub mod deprecated;

#[cfg(test)]
pub mod examples;
#[cfg(test)]
pub mod result_accuracy;
#[cfg(test)]
pub mod serialization;
#[cfg(test)]
pub mod large;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct BwtBlock1 {
    rank_check_point: [u64; 4],
    bwt_vector: [u128; 6],
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BwtBlock1NB {
    rank_check_point: [u64; 4],
    bwt_vector: [u128; 6],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct BwtBlock2 {
    rank_check_point: [u64; 5],
    bwt_vector: [u128; 6],
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BwtBlock2NB {
    rank_check_point: [u64; 5],
    bwt_vector: [u128; 6],
}

#[test]
fn test_byte_muck_size() {
    let block1 = BwtBlock1 {
        rank_check_point: [0; 4],
        bwt_vector: [0; 6],
    };
    let block1nb = BwtBlock1NB {
        rank_check_point: [0; 4],
        bwt_vector: [0; 6],
    };
    let block2 = BwtBlock2 {
        rank_check_point: [0; 5],
        bwt_vector: [0; 6],
    };
    let block2NB = BwtBlock2NB {
        rank_check_point: [0; 5],
        bwt_vector: [0; 6],
    };
    println!("block1: {}", std::mem::size_of::<BwtBlock1>());
    println!("block1nb: {}", std::mem::size_of::<BwtBlock1NB>());
    println!("block2: {}", std::mem::size_of::<BwtBlock2>());
    println!("block2nb: {}", std::mem::size_of::<BwtBlock2NB>());
}