mod blockchain;
mod node;

use crate::blockchain::{Blockchain,Block};
use std::env;
use std::time::Instant;
fn main(){
    let args:Vec<String>=env::args().collect();
    if args[1]=="server"{
        let blockchain=Blockchain::new(4);
        node::start_server(blockchain, "127.0.0.1:7878");
}else if args[1]=="client"{
    let mut block=Block::new(1,"hello world".to_string(),"0000....".to_string());
    let now=Instant::now();
    block.mine_block(4);
    println!("minning took {:.2?}",now.elapsed());
    node::send_block(&block, "127.0.0.1:7878");

}

}