use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct MerkleTreeNode {
    hash: String,
    left: Option<Box<MerkleTreeNode>>,
    right: Option<Box<MerkleTreeNode>>,
}

impl MerkleTreeNode {
    pub fn new(
        hash: String,
        left: Option<Box<MerkleTreeNode>>,
        right: Option<Box<MerkleTreeNode>>,
    ) -> Self {
        MerkleTreeNode { hash, left, right }
    }

    pub fn generate_tree(transactions: Vec<Transaction>) -> Option<Self> {
        let mut transaction_hashes: Vec<String> = transactions
            .into_iter()
            .map(|t| t.hash().to_string())
            .collect();

        if transaction_hashes.len() < 1 {
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
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0], &String::new()),
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
        while let Some(mut pair) = iter.next() {
            if pair.len() == 2 {
                parent_nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0].hash, &pair[1].hash),
                    left: Some(Box::new(pair[0].clone())),
                    right: Some(Box::new(pair[1].clone())),
                });
            } else {
                parent_nodes.push(MerkleTreeNode {
                    hash: MerkleTreeNode::generate_hash_of_children(&pair[0].hash, &pair[1].hash),
                    left: Some(Box::new(pair[0].clone())),
                    right: None,
                });
            }
        }

        MerkleTreeNode::construct_tree_from_leaves(parent_nodes)
    }

    fn generate_hash_of_children(left: &String, right: &String) -> String {
        let mut hasher = Sha256::new();

        hasher.update(format!("{}{}", left, right));

        format!("{:X}", hasher.finalize())
    }
}
