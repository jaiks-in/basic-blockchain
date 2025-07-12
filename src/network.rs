use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;
use crate::blockchain::{Blockchain,Block};
use serde_json;
pub fn start_server(mut blockchain:Blockchain,address:&str){
    let listener=TcpListener::bind(address).unwrap();
    println!("server running at{}",address);
    for stream in listener.incoming(){
        let stream=stream.unwrap();
        let mut blockchain=blockchain.clone();
        thread::spawn(move||{
            handle_connection(stream,&mut blockchain);
        });
    }

}
fn handle_connection(mut stream:TcpStream,blockchain:&mut Blockchain){
    let mut buffer=[0;2048];
    stream.read(&mut buffer).unwrap();
    let recieved=String::from_utf8_lossy(&buffer);
    println!("Recieved block{}\n",recieved);
    if let Ok(block)=serde_json::from_str::<Block>(&recieved){
        blockchain.add_block(block);
        println!("block added to chain");
    }
}
pub fn send_block(block:&Block,address:&str){
    let mut stream=TcpStream::connect(address).unwrap();
    let block_data=serde_json::to_string(block).unwrap();
    stream.write_all(block_data.as_bytes()).unwrap();
    println!("Block sent to {}",address);
}