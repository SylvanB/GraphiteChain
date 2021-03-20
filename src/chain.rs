use crate::block::{Block, BlockData};

const DIFFICULTY: usize = 2;

#[derive(Debug)]
pub struct Chain {
    pub difficulty: usize,
    chain: Vec<Block>,
}

impl Chain {
    fn get_genesis_block(difficulty: usize) -> Block {
        let mut genesis = Block::new(
            BlockData::new(String::from("Genesis Block")),
            String::from(""),
        );

        genesis.mine_block(difficulty);

        genesis
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn new() -> Self {
        Chain {
            difficulty: DIFFICULTY,
            chain: vec![Chain::get_genesis_block(DIFFICULTY)],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        block.generate_tree();
        self.chain.push(block);
    }

    pub fn validate_chain(&self) -> bool {
        if self.chain.len() == 1 {
            let genesis = self.chain.get(0).unwrap();
            return &genesis.calculate_hash() == genesis.get_hash()
                && genesis.get_prev_hash() == "";
        }

        let mut hashes_match = false;
        for x in 1..self.chain.len() {
            let curr = self.chain.get(x).unwrap();
            let prev = self.chain.get(x - 1).unwrap();

            hashes_match = curr.get_hash() == &curr.calculate_hash()
                && prev.get_hash() == curr.get_prev_hash();
        }

        hashes_match
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_hashes_in_chain() {
        // Crap test but yolo

        let mut chain = Chain::new();
        let block = Block::new(
            BlockData::new(String::from("Test")),
            chain.get_last_block().get_hash().clone(),
        );

        chain.add_block(block);

        assert!(chain.validate_chain());
    }
}
