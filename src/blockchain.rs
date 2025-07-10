use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.data, self.timestamp, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:X}", result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == &"0".repeat(difficulty) {
                println!("Block mined: {} (nonce: {})", self.hash, self.nonce);
                break;
            }
            self.nonce += 1;
        }
    }
}

#[derive(Debug,Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        genesis.mine_block(difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}
