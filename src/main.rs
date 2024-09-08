mod block;          // Import block module containing Block struct and its implementation.
mod blockchain;     // Import blockchain module containing Blockchain struct and its implementation.
mod transaction;    // Import transaction module containing Transaction struct and its implementation.

use crate::blockchain::Blockchain;

fn main() {
    // Create a new blockchain with the genesis block.
    let mut blockchain = Blockchain::new();
    
    // Add new blocks to the blockchain with some example data.
    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Second block".to_string());
    blockchain.add_block("Third block".to_string());

    // Print out each block in the blockchain.
    for block in blockchain.chain {
        println!("{:?}", block);
    }
}