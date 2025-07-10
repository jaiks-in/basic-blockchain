use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::blockchain::{Blockchain, Block};
use serde_json;

pub fn start_server(mut blockchain: Blockchain, address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut blockchain = blockchain.clone();

        thread::spawn(move || {
            handle_connection(stream, &mut blockchain);
        });
    }
}

fn handle_connection(mut stream: TcpStream, blockchain: &mut Blockchain) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let received = String::from_utf8_lossy(&buffer);
    println!("Received:\n{}", received);

    if let Ok(block) = serde_json::from_str::<Block>(&received) {
        blockchain.add_block(block);
        println!("✅ Block added! Current Chain: {:?}", blockchain.chain);
    }
}

pub fn send_block(block: &Block, address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let block_data = serde_json::to_string(block).unwrap();
    stream.write_all(block_data.as_bytes()).unwrap();
}
