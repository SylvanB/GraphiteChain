use graphite_chain::chain::Chain;
use graphite_chain::{
    block::{Block, BlockData},
    transaction::Transaction,
};

fn main() {
    let mut chain = Chain::new();

    let mut block = Block::new(
        BlockData::new(String::from("First Block")),
        chain.get_last_block().get_hash().clone(),
    );

    block.mine_block(chain.difficulty);
    chain.add_block(block);
    println!("Mined block 1!");

    let mut block2 = Block::new(
        BlockData::new(String::from("Second Block")),
        chain.get_last_block().get_hash().clone(),
    );

    block2.mine_block(chain.difficulty);
    chain.add_block(block2);
    println!("Mined block 2!");

    let mut block3 = Block::new(
        BlockData::new(String::from("ThirdBlock")),
        chain.get_last_block().get_hash().clone(),
    );

    block3.mine_block(chain.difficulty);
    chain.add_block(block3);
    println!("Mined block 3!");

    println!("Chain is valid: {}", chain.validate_chain());

    println!("Quick Merkle Tree Hash Generation Check");

    let mut block4 = Block::new(
        BlockData::new(String::from("FourthBlock")),
        chain.get_last_block().get_hash().clone(),
    );

    block4.add_transaction(Transaction::new(
        "source".to_string(),
        "destiation".to_string(),
        0.1,
        0.0,
    ));

    block4.add_transaction(Transaction::new(
        "source".to_string(),
        "destiation".to_string(),
        0.1,
        0.0,
    ));

    block4.add_transaction(Transaction::new(
        "source".to_string(),
        "destiation".to_string(),
        0.1,
        0.0,
    ));

    block4.add_transaction(Transaction::new(
        "source".to_string(),
        "destiation".to_string(),
        0.1,
        0.0,
    ));

    println!("Merkle Root hash generated from:\n{:#?}", &block4);
    println!(
        "Merkle Tree Root Hash for block: {}",
        &block4.generate_merkle_tree_hash()
    );

    println!("{:#?}", chain);
}
