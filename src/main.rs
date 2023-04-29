pub mod blockchain;
pub mod block;

use crate::blockchain::Blockchain;
use crate::block::Block;

fn main() {
    let _difficulty: i128 = 0x0000ffffffffffffffffffffffffffff;

    let mut blockchain = Blockchain::new();

    blockchain.add_block(Block::new(1, 0, vec![0; 32], [0; 32], 42));
    blockchain.add_block(Block::new(2, 0, vec![0; 32], [0; 32], 128));

    for block in blockchain.blocks.iter() {
        println!("{:?}", block);
    }
}
