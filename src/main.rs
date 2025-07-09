use chrono::prelude::*;
use sha2::{Sha256,Digest};
use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug,Clone)]
struct Block{
    index:u64,
    timestamp:u128,
    data:String,
    previous_hash:String,
    hash:String,
    nonce:u64,
}
impl Block{
    fn new (index:u64,data:String,previous_hash:String)->Self{
        let timestamp=Utc::now().timestamp_millis() as u128;
        let mut block=Block{
            index,timestamp,data,previous_hash,hash:String::new(),nonce:0,
        };
        block.hash=block.calculate_hash();
        block
    }
    fn calculate_hash(&self)->String{
        let block_data=format!("{}{}{}{}{}",self.index,self.data,self.timestamp,self.previous_hash,self.nonce);
        let mut hasher=Sha256::new();
        hasher.update(block_data);
        let result=hasher.finalize();
        format!("{:X}",result)
    }
    fn mine_block(&mut self ,difficulty:usize){
        loop{
            self.hash=self.calculate_hash();
            if &self.hash[..difficulty]==&"0".repeat(difficulty){
                print!("block mined:{}(nonce:{})",self.hash,self.nonce);
                break;
            }
            self.nonce+=1;
        }
    }
}
struct Blockchain{
    chain:Vec<Block>,
    difficulty:usize,
}
impl Blockchain{
    fn new(difficulty:usize)->Self{
        let mut genesis_block=Block::new(0,"Genesis Block".to_string(),"0".to_string());
        genesis_block.mine_block(difficulty);
        Blockchain{
            chain:vec![genesis_block],
            difficulty,
        }
    }
    fn get_last_block(&self)->&Block{
        self.chain.last().expect("Blockchain is empty")
    }
    fn add_block(&mut self,data:String){
        let last_block=self.get_last_block();
        let mut new_block=Block::new(
            last_block.index+1,
            data,
            last_block.hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }
    fn print_chain(&self){
        for block in &self.chain{
            let serialized=serde_json::to_string_pretty(&block).unwrap();
            println!("{}",serialized);
        }
    }
}
fn main() {
    let mut my_blockchain=Blockchain::new(4);
    my_blockchain.add_block("data 1".to_string());
    my_blockchain.print_chain();
}
