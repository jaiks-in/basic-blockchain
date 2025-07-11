mod blockchain;
mod node;

use crate::blockchain::{Block, Blockchain, Transaction};
use std::env;
use std::time::Instant;
fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()<2{
        print!("usage:cargo run -- client");
        return;
    }
    if args[1]=="server"{
        let mut  blockchain=Blockchain::new(4);
        let tx1=Transaction{
            sender:"abc".to_string(),
            receiver:"xyz".to_string(),
            amount:50
        };
        blockchain.add_transaction(tx1);
        let tx2=Transaction{
            sender:"xyz".to_string(),
            receiver:"hjk".to_string(),
            amount:24,
        };
        blockchain.add_transaction(tx2);
        blockchain.mine_pending_transactions("miner ".to_string());
        for block in &blockchain.chain{
            print!("{:?}",block);
        }
        println!("\nChain valid? {}", blockchain.is_valid());
        node::start_server(blockchain, "127.0.0.1:7878");
}

}