use crate::block::Block;

/// Represents the blockchain, which is a chain of blocks.
pub struct Blockchain {
    pub chain: Vec<Block>,   // Vector containing all the blocks in the blockchain.
    pub difficulty: usize,   // Mining difficulty (number of leading zeros required in the hash).
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    pub fn new() -> Blockchain {
        // Initialize the blockchain with a genesis block and set the difficulty level.
        let mut blockchain = Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            difficulty: 4,  // Example difficulty level; can be adjusted.
        };
        blockchain
    }

    /// Creates the genesis block, which is the first block in the blockchain.
    /// The genesis block is unique and has no previous hash (set to "0").
    fn create_genesis_block() -> Block {
        Block::new(0, "Genesis Block".to_string(), "0".to_string())
    }

    /// Adds a new block to the blockchain with the provided data.
    /// The new block's previous hash is set to the hash of the last block in the chain.
    pub fn add_block(&mut self, data: String) {
        // Get the hash of the last block in the chain to link the new block.
        let previous_hash = self.chain.last().unwrap().hash.clone();
        
        // Create and mine the new block.
        let mut block = Block::new(self.chain.len() as u32, data, previous_hash);
        block.mine_block(self.difficulty);
        
        // Add the mined block to the blockchain.
        self.chain.push(block);
    }
}