use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use p256::ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer, signature::Verifier};
use rand::rngs::OsRng;
use crate::network::send_block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:X}", hasher.finalize())
    }

    pub fn sign_transaction(&mut self, private_key: &SigningKey) {
        let hash = self.calculate_hash();
        let signature: p256::ecdsa::Signature = private_key.sign(hash.as_bytes());
        let sig_base64 = general_purpose::STANDARD.encode(signature.to_der());
        self.signature = Some(sig_base64);
    }

    pub fn is_valid(&self, public_key: &VerifyingKey) -> bool {
        if self.sender == "System" { return true; }
        if self.signature.is_none() { return false; }
        let hash = self.calculate_hash();
        let sig_bytes = general_purpose::STANDARD.decode(self.signature.as_ref().unwrap()).unwrap();
        let sig = Signature::from_der(&sig_bytes).unwrap();
        public_key.verify(hash.as_bytes(), &sig).is_ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transaction: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transaction: Vec<Transaction>, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u128,
            transaction,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let tx_data = serde_json::to_string(&self.transaction).unwrap();
        let data = format!("{}{}{}{}{}", self.index, tx_data, self.timestamp, self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:X}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == &"0".repeat(difficulty) {
                println!("⛏️ Mined block: {} (nonce: {})", self.hash, self.nonce);
                break;
            }
            self.nonce += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transaction: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis = Block::new(0, vec![], "0".to_string());
        genesis.mine_block(difficulty);
        Blockchain {
            chain: vec![genesis],
            difficulty,
            pending_transaction: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transaction.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner: String, peer: &str) {
        let reward = Transaction {
            sender: "System".to_string(),
            receiver: miner,
            amount: 100,
            signature: None,
        };
        self.pending_transaction.push(reward);
        let mut block = Block::new(self.chain.len() as u64, self.pending_transaction.clone(), self.chain.last().unwrap().hash.clone());
        block.mine_block(self.difficulty);
        self.chain.push(block.clone());
        send_block(&block, peer);
        self.pending_transaction.clear();
    }

    pub fn add_block(&mut self, block: Block) {
        let last = self.chain.last().unwrap();
        if block.previous_hash != last.hash || block.calculate_hash() != block.hash {
            println!("❌ Invalid block!");
            return;
        }
        self.chain.push(block);
        println!("✅ Block added to chain!");
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) {
        if new_chain.len() > self.chain.len() {
            self.chain = new_chain;
            println!("✅ Chain replaced!");
        } else {
            println!("❌ Received chain invalid or not longer.");
        }
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}
