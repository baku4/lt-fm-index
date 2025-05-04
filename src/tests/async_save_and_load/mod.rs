use crate::{LtFmIndex, Position, Block};
use crate::blocks::{Block2, Block3, Block4, Block5, Block6};
use crate::tests::random_data::{
    gen_rand_chr_list,
    gen_rand_text,
};
use std::pin::Pin;

async fn assert_serializing_is_success<P: Position, B: Block<P> + std::cmp::PartialEq>(
    chr_list: &Vec<u8>,
    text: Vec<u8>,
    ltks: u32,
    sasr: u64,
) {
    if B::MAX_CHR < chr_list.len() as u32 {
        println!("          pass");
        return;
    }
    let characters_by_index = chr_list.chunks(1).map(|c| c).collect::<Vec<_>>();
    let lt_fm_index = LtFmIndex::<P, B>::build(
        text,
        &characters_by_index,
        P::from_u64(sasr),
        ltks,
    ).unwrap();
    let mut buffer = Vec::new();
    lt_fm_index.async_save_to(Pin::new(&mut buffer)).await.unwrap();

    let loaded: LtFmIndex::<P, B> = LtFmIndex::async_load_from(Pin::new(&mut &buffer[..])).await.unwrap();
    assert_eq!(lt_fm_index, loaded);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn async_save_and_load() {
    let range_chr_count = 2..4;
    let text_min_len = 100;
    let text_max_len = 300;
    let n_text = 2;
    let ltks = 3;
    let sasr = 2;

    for chr_count in range_chr_count {
        println!("- Chr count: {}", chr_count);
        for i in 0..n_text {
            println!("  - text: {}/{}", i+1, n_text);
            let chr_list = gen_rand_chr_list(chr_count);
            let text = gen_rand_text(&chr_list, text_min_len, text_max_len);

            macro_rules! test_type_of {
                ( $p: ty, $b: ident, $v: ty ) => {
                    assert_serializing_is_success::<$p, $b::<$v>>(
                        &chr_list,
                        text.clone(),
                        ltks,
                        sasr,
                    ).await;
                };
            }
            macro_rules! of_position_for_blocks {
                ( $( $p:ty ),* ) => {
                    $(
                        println!("      - Block: Block2");
                        for_vectors!($p, Block2);
                        println!("      - Block: Block3");
                        for_vectors!($p, Block3);
                        println!("      - Block: Block4");
                        for_vectors!($p, Block4);
                        println!("      - Block: Block5");
                        for_vectors!($p, Block5);
                        println!("      - Block: Block6");
                        for_vectors!($p, Block6);
                    )*
                };
            }
            macro_rules! for_vectors {
                ( $( $p: ty, $b: ident ),* ) => {
                    $(
                        println!("        - Vector: u32");
                        test_type_of!($p, $b, u32);
                        println!("        - Vector: u64");
                        test_type_of!($p, $b, u64);
                        println!("        - Vector: u128");
                        test_type_of!($p, $b, u128);
                    )*
                };
            }
            println!("    - Position: u32");
            of_position_for_blocks!(u32);
            println!("    - Position: u64");
            of_position_for_blocks!(u64);
        }
    }
}
