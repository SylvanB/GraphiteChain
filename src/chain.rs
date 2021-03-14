use crate::block::{Block, BlockData};

#[derive(Debug)]
pub struct Chain {
    chain: Vec<Block>,
}

impl Chain {
    fn get_genesis_block() -> Block {
        Block::new(
            BlockData::new(String::from("Genesis Block")),
            String::from(""),
        )
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn new() -> Self {
        Chain {
            chain: vec![Chain::get_genesis_block()],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}
