use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transaction:Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transaction: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u128;
        Block {
            index,
            timestamp,
            transaction,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let tx_data=serde_json::to_string(&self.transaction).unwrap();
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, tx_data, self.timestamp, self.previous_hash, self.nonce
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
    pub pending_transaction:Vec<Transaction>,
}

impl Blockchain {
    pub fn mine_pending_transactions(&mut self,miner_address:String){
        //miners will get reward
        let reward_tx=Transaction{
            sender:"System".to_string(),
            receiver:miner_address,
            amount:100
        };
        self.pending_transaction.push(reward_tx);
        let mut block=Block::new(
            self.chain.len() as u64,
            self.pending_transaction.clone(),
            self.chain.last().unwrap().hash.clone(),
        );
        block.mine_block(self.difficulty);
        self.chain.push(block);
        self.pending_transaction.clear();
    }
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
    pub fn add_transaction(&mut self,transaction:Transaction){
        self.pending_transaction.push(transaction);
    }
   pub fn new(difficulty: usize) -> Self {
    let mut genesis = Block::new(0, vec![], "0".to_string());
    genesis.mine_block(difficulty);
    Blockchain {
        chain: vec![genesis],
        difficulty,
        pending_transaction: vec![],
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
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Transaction{
    pub sender:String,
    pub receiver:String,
    pub amount:u64,
}
