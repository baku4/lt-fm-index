use crate::*;
use super::random_text::*;
use std::time::Instant;

fn print_generate_large_index_no() {
    for i in 30..34 {
        println!("# {}", i);
        let n = 1usize << i;

        let t = Instant::now();
        let text = gen_rand_text(&NO_STEMS, n..n);
        println!("Text size: {}, {}s", n, t.elapsed().as_secs_f64());

        let t = Instant::now();
        let lt_fm_index = LtFmIndexBuilderDep::new()
            .text_type_is_nucleotide_only()
            .build(text)
            .unwrap();
        println!("LtFmIndex generated, {}s", t.elapsed().as_secs_f64());

        let t = Instant::now();
        let mut buffer = Vec::new();
        lt_fm_index.save_to(&mut buffer).unwrap();
        println!("LtFmIndex saved, {} bytes, {}s", buffer.len(), t.elapsed().as_secs_f64());

        let t = Instant::now();
        let cursor = std::io::Cursor::new(buffer);
        let _ = LtFmIndexDep::load_from(cursor).unwrap();
        println!("LtFmIndex loaded, {}s", t.elapsed().as_secs_f64());
    }
}
