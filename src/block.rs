use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a block in the blockchain.
#[derive(Debug)]
pub struct Block {
    pub index: u32,            // Index of the block in the blockchain.
    pub timestamp: u64,        // Timestamp when the block was created.
    pub data: String,          // Data stored in the block (e.g., transaction details).
    pub previous_hash: String, // Hash of the previous block in the chain.
    pub hash: String,          // Current block's hash.
    pub nonce: u64,            // Nonce used for mining the block (proof-of-work).
}

impl Block {
    /// Creates a new block with the given index, data, and previous block's hash.
    pub fn new(index: u32, data: String, previous_hash: String) -> Block {
        // Get the current time in seconds since UNIX epoch.
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create a block with empty hash and nonce initialized to 0.
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        // Calculate the block's hash based on its properties.
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block based on its properties.
    pub fn calculate_hash(&self) -> String {
        format!(
            "{:x}",
            md5::compute(format!(
                "{}{}{}{}{}",
                self.index, self.timestamp, self.data, self.previous_hash, self.nonce
            ))
        )
    }

    /// Mines the block by finding a hash that meets the difficulty criteria.
    /// The difficulty is represented by the number of leading zeros in the hash.
    pub fn mine_block(&mut self, difficulty: usize) {
        // Target string with `difficulty` leading zeros.
        let target = "0".repeat(difficulty);
        
        // Increment nonce until the hash starts with the required number of zeros.
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        
        // Output the mined block's hash.
        println!("Block mined: {}", self.hash);
    }
}