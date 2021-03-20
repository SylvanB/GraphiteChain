use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct MerkleTreeNode {
    pub hash: String,
    pub left: Option<Box<MerkleTreeNode>>,
    pub right: Option<Box<MerkleTreeNode>>,
}

// Yes this is pretty naive
// Yes I need to rewrite this
impl MerkleTreeNode {
    pub fn new(
        hash: String,
        left: Option<Box<MerkleTreeNode>>,
        right: Option<Box<MerkleTreeNode>>,
    ) -> Self {
        MerkleTreeNode { hash, left, right }
    }

    pub fn generate_tree(transactions: Vec<Transaction>) -> Option<Self> {
        let transaction_hashes: Vec<String> = transactions
            .into_iter()
            .map(|t| t.hash().to_string())
            .collect();

        if transaction_hashes.is_empty() {
            return None;
        }

        let mut nodes = vec![];
        let mut iter = transaction_hashes.chunks(2);
        while let Some(pair) = iter.next() {
            if pair.len() == 2 {
                nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0], &pair[1]),
                    left: Some(Box::new(MerkleTreeNode::new(
                        pair[0].to_string(),
                        None,
                        None,
                    ))),
                    right: Some(Box::new(MerkleTreeNode::new(
                        pair[1].to_string(),
                        None,
                        None,
                    ))),
                });
            } else {
                nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0], &""),
                    left: Some(Box::new(MerkleTreeNode::new(
                        pair[0].to_string(),
                        None,
                        None,
                    ))),
                    right: None,
                });
            }
        }

        Some(MerkleTreeNode::construct_tree_from_leaves(nodes))
    }

    fn construct_tree_from_leaves(mut nodes: Vec<MerkleTreeNode>) -> MerkleTreeNode {
        if nodes.len() == 1 {
            return nodes.remove(0);
        }

        let mut parent_nodes = vec![];
        let mut iter = nodes.chunks(2);
        while let Some(pair) = iter.next() {
            if pair.len() == 2 {
                parent_nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0].hash, &pair[1].hash),
                    left: Some(Box::new(pair[0].clone())),
                    right: Some(Box::new(pair[1].clone())),
                });
            } else {
                parent_nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0].hash, &""),
                    left: Some(Box::new(pair[0].clone())),
                    right: None,
                });
            }
        }

        MerkleTreeNode::construct_tree_from_leaves(parent_nodes)
    }

    fn generate_hash_of_children(left: &str, right: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(format!("{}{}", left, right));

        format!("{:X}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Block, BlockData};

    #[test]
    fn it_generates_a_valid_tree_with_even_amount_of_leaves() {
        // Probably be nicer to abstract this test away from the Block/Transaction implementation

        let mut block = Block::new(BlockData::new("test"), "");

        let t1 = Transaction::new("src", "dest", 0.01, 0.01);
        let t2 = Transaction::new("src", "dest", 0.02, 0.02);

        let t1_hash = t1.hash().to_string();
        let t2_hash = t2.hash().to_string();

        block.add_transaction(t1);
        block.add_transaction(t2);

        let root_hash = block.generate_tree().unwrap().hash;
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", t1_hash, t2_hash));

        let expeceted_hash = format!("{:X}", hasher.finalize());
        assert!(root_hash == expeceted_hash);
    }

    #[test]
    fn it_generates_a_valid_tree_with_odd_amount_of_leaves() {
        let mut block = Block::new(BlockData::new("test"), "");

        let t1 = Transaction::new("src", "dest", 0.01, 0.01);

        let t1_hash = t1.hash().to_string();

        block.add_transaction(t1);

        let root_hash = block.generate_tree().unwrap().hash;
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", t1_hash, ""));

        let expeceted_hash = format!("{:X}", hasher.finalize());
        assert!(root_hash == expeceted_hash);
    }
}
