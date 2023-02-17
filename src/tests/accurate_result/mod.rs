use crate::{LtFmIndex, Position, Block};
use crate::blocks::{Block2, Block3, Block4, Block5, Block6};
use crate::tests::{
    random_data::{
        gen_rand_chr_list,
        gen_rand_text,
        gen_rand_pattern,
    },
    result_answer::{
        get_fmindex_of_other_crate,
        get_sorted_locations,
    },
};

fn assert_accurate_lt_fm_index<P: Position, B: Block<P>>(
    chr_list: &Vec<u8>,
    text: Vec<u8>,
    patterns: &Vec<Vec<u8>>,
    answers: &Vec<Vec<u64>>,
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
    patterns.iter().zip(answers.iter()).for_each(|(pattern, answer)| {
        let mut result: Vec<u64> = lt_fm_index.locate(pattern).into_iter().map(|x| x.as_u64()).collect();
        result.sort();
        assert_eq!(&result, answer);
    });
}

#[test]
fn result_is_accurate() {
    let range_chr_count = 2..4;
    let text_min_len = 100;
    let text_max_len = 300;
    let n_text = 2;
    let pattern_min_len = 1;
    let pattern_max_len = 10;
    let n_pattern = 100;
    let ltks = 3;
    let sasr = 2;

    // *** For long and thorough test ***
    // 
    // let n_text = 10;
    // let range_chr_count = 2..63;
    // 

    for chr_count in range_chr_count {
        println!("- Chr count: {}", chr_count);
        for i in 0..n_text {
            println!("  - text: {}/{}", i+1, n_text);
            let chr_list = gen_rand_chr_list(chr_count);
            let text = gen_rand_text(&chr_list, text_min_len, text_max_len);

            let patterns: Vec<Vec<u8>> = (0..n_pattern).map(|_| {
                gen_rand_pattern(&text, pattern_min_len, pattern_max_len)
            }).collect();
            let answers: Vec<Vec<u64>> = {
                let fm_index = get_fmindex_of_other_crate(&text);
                patterns.iter().map(|pattern| {
                    get_sorted_locations(&fm_index, pattern)
                }).collect()
            };
            macro_rules! test_type_of {
                ( $p: ty, $b: ident, $v: ty ) => {
                    assert_accurate_lt_fm_index::<$p, $b::<$v>>(
                        &chr_list,
                        text.clone(),
                        &patterns,
                        &answers,
                        ltks,
                        sasr,
                    )
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
