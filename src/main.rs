use graphite_chain::block::{Block, BlockData};
use graphite_chain::chain::Chain;

fn main() {
    let mut chain = Chain::new();

    let block = Block::new(
        BlockData::new(String::from("First Block")),
        chain.get_last_block().hash.clone(),
    );

    chain.add_block(block);

    println!("{:#?}", chain);
}
