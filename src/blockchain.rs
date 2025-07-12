use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use p256::ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer, signature::Verifier};
use rand::rngs::OsRng;

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
        let result = hasher.finalize();
        format!("{:X}", result)
    }

    pub fn sign_transaction(&mut self, private_key: &SigningKey) {
        let hash = self.calculate_hash();
        let signature:p256::ecdsa::Signature = private_key.sign(hash.as_bytes());
        let signature_base64 = general_purpose::STANDARD.encode(signature.to_der());
        self.signature = Some(signature_base64);
    }

    pub fn is_valid(&self, public_key: &VerifyingKey) -> bool {
        if self.sender == "System" {
            return true;
        }
        if self.signature.is_none() {
            println!("❌ Transaction signature missing!");
            return false;
        }
        let hash = self.calculate_hash();
        let signature_bytes = general_purpose::STANDARD.decode(self.signature.as_ref().unwrap()).unwrap();
        let sig = Signature::from_der(&signature_bytes).unwrap();

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
        let tx_data = serde_json::to_string(&self.transaction).unwrap();
        let block_data = format!("{}{}{}{}{}", self.index, tx_data, self.timestamp, self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:X}", result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if &self.hash[..difficulty] == &"0".repeat(difficulty) {
                println!("⛏️ Block mined: {} (nonce: {})", self.hash, self.nonce);
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

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction {
            sender: "System".to_string(),
            receiver: miner_address,
            amount: 100,
            signature: None,
        };
        self.pending_transaction.push(reward_tx);

        let mut block = Block::new(self.chain.len() as u64, self.pending_transaction.clone(), self.chain.last().unwrap().hash.clone());
        block.mine_block(self.difficulty);

        self.chain.push(block);
        self.pending_transaction.clear();
    }
    pub fn add_block(&mut self,block:Block){
        let last_block=self.chain.last().unwrap();
        if block.previous_hash!=last_block.hash{
            println!("Block rejected Previous hash mismatched");
            return;
        }
        if block.calculate_hash()!=block.hash{
            println!("block rejected invalid block hash");
        }
        self.chain.push(block);
        println!("Block Successfully added to Chain!");
    }
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash {
                return false;
            }
            if current.calculate_hash() != current.hash {
                return false;
            }
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}

pub struct Wallet {
    pub private_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let private_key = SigningKey::random(&mut OsRng);
        let public_key = VerifyingKey::from(&private_key);
        Wallet {
            private_key,
            public_key,
        }
    }
}
