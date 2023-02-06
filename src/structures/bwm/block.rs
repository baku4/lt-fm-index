use super::{BwtBlock, VectorBit};

struct BlockEncoded4<V: VectorBit> {
    rank_check_point: [u64; 4],
    bwt_vector: [V; 2],
}

impl<V: VectorBit> BwtBlock for BlockEncoded4<V> {
    type Bit = V;

    fn new_with_bwt_text(bwt_text: crate::core::Text) -> Vec<Self> {
        let mut chunk_count = bwt_text.len() / Self::Bit::LENGTH as usize;
        let rem = bwt_text.len() % Self::Bit::LENGTH as usize;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            Self::Bit::LENGTH as usize - rem
        };

        let mut rank_checkpoint = [0; chunk_count]
        let mut blocks: Vec<B> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(B::Bit::LENGTH).for_each(|text_chunk| {
            let block_rank_checkpoint = rank_checkpoint.clone();
            
            let bwt_vector = W::encoding_text_chunk(text_chunk, &mut rank_checkpoint);

            let block = W::new(block_rank_checkpoint, bwt_vector);
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = W::new_last(rank_checkpoint);
            blocks.push(last_block);
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.add_offset(last_offset);
        }

        blocks
    }
    fn get_rank(&self, rem: u64, chridx: u8) -> u64 {
        // FIXME:
        0
    }
    fn get_rank_and_chridx_of_rem(&self, rem: u64) -> (u64, u8) {
        // FIXME:
        (0, 0)
    }
}