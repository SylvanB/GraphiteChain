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
    pub data: BlockData,
    pub hash: String,
    pub prev_hash: String,
    pub timestamp: u128,
}

impl Block {
    pub fn new(data: BlockData, prev_hash: String) -> Self {
        let unix_epoch_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let mut block = Block {
            data,
            hash: String::new(),
            prev_hash,
            timestamp: unix_epoch_time.as_millis(),
        };

        block.hash = Block::calculate_hash(&block);

        block
    }

    pub fn calculate_hash(block: &Block) -> String {
        let mut hasher = Sha256::new();

        let data = format!("{}{}{}", block.prev_hash, block.timestamp, block.data);

        hasher.update(data);
        let hash = hasher.finalize();

        format!("{:X}", hash)
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

        let hash_content =
            format!("{}{}{}", block.prev_hash, block.timestamp, block.data).into_bytes();
        let hash = sha2::Sha256::digest(&hash_content);

        assert_eq!(format!("{:X}", hash), block.hash);
    }
}
