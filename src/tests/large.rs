use crate::*;
use super::random_text::*;
use std::time::Instant;

fn print_generate_large_index_no() {
    for i in 30..33 {
        println!("# {}", i);
        let n = 1usize << i;

        let t = Instant::now();
        let text = rand_text_with_length(&UTF8_OF_NO, n);
        println!("Text size: {}, {}s", n, t.elapsed().as_secs_f64());

        let t = Instant::now();
        let lt_fm_index = LtFmIndexBuilder::new()
            .use_nucleotide_only()
            .build(text);
        println!("LtFmIndex generated: {} bytes, {}s", lt_fm_index.inner_bytes_size(), t.elapsed().as_secs_f64());

        let t = Instant::now();
        let mut buffer = Vec::new();
        lt_fm_index.save_to(&mut buffer).unwrap();
        println!("LtFmIndex saved, {}s", t.elapsed().as_secs_f64());

        let t = Instant::now();
        let cursor = std::io::Cursor::new(buffer);
        let loaded = LtFmIndex::load_from(cursor).unwrap();
        println!("LtFmIndex loaded, {}s", t.elapsed().as_secs_f64());
    }
}
