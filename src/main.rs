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
}
impl Block{
    fn new (index:u64,data:String,previous_hash:String)->Self{
        let timestamp=Utc::now().timestamp_millis() as u128;
        let mut block=Block{
            index,timestamp,data,previous_hash,hash:String::new(),
        };
        block.hash=block.calculate_hash();
        block
    }
    fn calculate_hash(&self)->String{
        let block_data=format!("{}{}{}{}",self.index,self.data,self.timestamp,self.previous_hash);
        let mut hasher=Sha256::new();
        hasher.update(block_data);
        let result=hasher.finalize();
        format!("{:X}",result)
    }
}
struct Blockchain{
    chain:Vec<Block>,
}
impl Blockchain{
    fn new()->Self{
        let genesis_block=Block::new(0,"Genesis Block".to_string(),"0".to_string());
        Blockchain{
            chain:vec![genesis_block],
        }
    }
    fn get_last_block(&self)->&Block{
        self.chain.last().expect("Blockchain is empty")
    }
    fn add_block(&mut self,data:String){
        let last_block=self.get_last_block();
        let new_block=Block::new(
            last_block.index+1,
            data,
            last_block.hash.clone(),
        );
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
    let mut my_blockchain=Blockchain::new();
    my_blockchain.add_block("data 1".to_string());
    my_blockchain.print_chain();
}
