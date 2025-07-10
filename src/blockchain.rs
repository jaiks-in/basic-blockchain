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
    pub fn is_valid(&self)->bool{
        for i in 1..self.chain.len(){
            let current=&self.chain[i];
            let previous=&self.chain[i-1];
            if current.previous_hash!=previous.hash{
                return false;
            }
            if current.calculate_hash()!=current.hash{
                return false
            }
        }
        true
    }
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        genesis.mine_block(difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
        }
    }
    pub fn replace_chain(&mut self,new_chain:Vec<Block>){
        if new_chain.len()>self.chain.len(){
            self.chain=new_chain;
            println!("chain replaced wit longer valid chain");
        }else{
            println!("Received chain is invalid or shorter .Ignored ");
        }
    }
    pub fn add_block(&mut self, block: Block) {
        let last_block=self.chain.last().unwrap();
        if block.previous_hash!=last_block.hash{
            print!("block rejected !previous hash mismatched! hash of previous block is {}not equal to  last block{}",block.previous_hash,last_block.hash);
            return;
        }
        if block.calculate_hash()!=block.hash{
            println!("Block rejected ,invalid hash");
            return;
        }
        self.chain.push(block);
        println!("block added succesfully");
    }
}
