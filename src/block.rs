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
    pub fn new<T>(message: T) -> Self
    where
        T: Into<String>,
    {
        BlockData {
            message: message.into(),
        }
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
    pub fn new<T>(data: BlockData, prev_hash: T) -> Self
    where
        T: Into<String>,
    {
        let unix_epoch_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut block = Block {
            data,
            transactions: vec![],
            hash: String::new(),
            prev_hash: prev_hash.into(),
            timestamp: unix_epoch_time.as_micros(),
            nonce: 0,
            transaction_merkle_tree: None,
        };

        block.hash = block.calculate_hash();

        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let transaction_root = match &self.transaction_merkle_tree {
            None => String::new(),
            Some(r) => r.hash.clone(),
        };

        let data = format!(
            "{}{}{}{}{}",
            self.prev_hash, self.timestamp, self.data, self.nonce, transaction_root
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
        self.transaction_merkle_tree = self.generate_tree();

        let target_substring = vec!["0"; difficulty].into_iter().collect::<String>();
        if !self.hash.is_empty() && &self.hash[0..difficulty] == target_substring.as_str() {
            return;
        }

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

    pub fn generate_tree(&self) -> Option<MerkleTreeNode> {
        MerkleTreeNode::generate_tree(self.transactions.clone())
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
    fn it_generates_a_valid_hash_according_to_difficulty_after_mining() {
        let mut block = Block::new(
            BlockData::new(String::from("test")),
            String::from("prev_hash"),
        );
        let diff = 1;
        block.mine_block(diff);

        let expected_format = format!("{}", vec!["0"; diff].into_iter().collect::<String>());
        let valid_hash = block.hash.contains(&expected_format);

        println!("{}", block.hash);

        assert!(valid_hash);
    }

    #[test]
    fn it_generates_hash_based_on_block_content_after_mining() {
        let mut block = Block::new(
            BlockData::new(String::from("test")),
            String::from("prev_hash"),
        );
        let diff = 1;
        block.mine_block(diff);

        let transaction_root = match &block.transaction_merkle_tree {
            None => String::new(),
            Some(r) => r.hash.clone(),
        };

        let expected_hash_content = format!(
            "{}{}{}{}{}",
            block.prev_hash, block.timestamp, block.data, block.nonce, transaction_root
        );

        let mut hasher = Sha256::new();
        hasher.update(expected_hash_content);

        let expected_hash = format!("{:X}", hasher.finalize());

        assert!(expected_hash == block.hash);
    }
}
