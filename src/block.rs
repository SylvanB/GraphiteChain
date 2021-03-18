use crate::merkle_tree::MerkleTreeNode;
use crate::transaction::Transaction;
use sha2::{Digest, Sha256};
use std::fmt::Display;
use std::time::SystemTime;

#[derive(Debug)]
pub struct BlockData {
    pub message: String,
}

impl BlockData {
    pub fn new(message: String) -> Self {
        BlockData { message }
    }
}

impl Display for BlockData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub struct Block {
    data: BlockData,
    transactions: Vec<Transaction>,
    hash: String,
    prev_hash: String,
    timestamp: u128,
    nonce: u128,
    transaction_merkle_tree: Option<MerkleTreeNode>,
}

impl Block {
    pub fn new(data: BlockData, prev_hash: String) -> Self {
        let unix_epoch_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut block = Block {
            data,
            transactions: vec![],
            hash: String::new(),
            prev_hash,
            timestamp: unix_epoch_time.as_micros(),
            nonce: 0,
            transaction_merkle_tree: None,
        };

        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let data = format!(
            "{}{}{}{}",
            self.prev_hash, self.timestamp, self.data, self.nonce
        );

        hasher.update(data);
        let hash = hasher.finalize();

        format!("{:X}", hash)
    }

    pub fn get_prev_hash(&self) -> &String {
        &self.prev_hash
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_block_data(&self) -> &BlockData {
        &self.data
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target_substring = vec!["0"; difficulty].into_iter().collect::<String>();
        loop {
            match &self.hash[0..difficulty] != target_substring.as_str() {
                false => break,
                true => {
                    self.nonce += 1;
                    self.hash = self.calculate_hash();
                }
            }
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn generate_tree(&self) -> MerkleTreeNode {
        MerkleTreeNode::generate_tree(self.transactions.clone()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_block_with_given_data() {
        let block = Block::new(BlockData::new(String::from("test")), String::from(""));

        assert_eq!(
            BlockData::new(String::from("test")).message,
            block.data.message
        );
    }

    #[test]
    fn it_creates_a_block_with_given_prev_hash() {
        let block = Block::new(
            BlockData::new(String::from("test")),
            String::from("prev_hash"),
        );

        assert_eq!("prev_hash", block.prev_hash);
    }

    #[test]
    fn it_generates_an_sha256_hash() {
        let block = Block::new(
            BlockData::new(String::from("test")),
            String::from("prev_hash"),
        );

        let hash_content = format!(
            "{}{}{}{}",
            block.prev_hash, block.timestamp, block.data, block.nonce
        )
        .into_bytes();
        let hash = sha2::Sha256::digest(&hash_content);

        assert_eq!(format!("{:X}", hash), block.hash);
    }

    #[test]
    fn it_generates_a_valid_hash_after_mining() {
        let mut block = Block::new(
            BlockData::new(String::from("test")),
            String::from("prev_hash"),
        );
        let diff = 1;
        block.mine_block(diff);

        let expected_format = format!("{}", vec!["0"; diff].into_iter().collect::<String>());
        let did_generate_correct_hash = block.hash.contains(&expected_format);

        println!("{}", block.hash);

        assert!(did_generate_correct_hash == true);
    }
}
