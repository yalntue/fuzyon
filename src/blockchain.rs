use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, 0, vec![0; 32], [0; 32], 0);
        Self {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        let previous_hash = self.blocks.last().unwrap().hash;
        assert_eq!(block.previous_hash, previous_hash);
        self.blocks.push(block);
    }
}
