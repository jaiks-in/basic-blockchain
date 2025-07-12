use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use crate::blockchain::{Block, Blockchain};
use crate::peer::PeerList;
use serde_json;

pub fn start_server(mut blockchain: Blockchain, peer_list: PeerList, address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    println!("🛰️ Server on {}", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut bc_clone = blockchain.clone();
        let peers = peer_list.clone();
        thread::spawn(move || {
            handle_connection(stream, &mut bc_clone, peers);
        });
    }
}

fn handle_connection(mut stream: TcpStream, blockchain: &mut Blockchain, peer_list: PeerList) {
    let mut buffer = [0; 8192];
    stream.read(&mut buffer).unwrap();
    let received = String::from_utf8_lossy(&buffer);

    if received.starts_with("peer:") {
        let peer_addr = received.replace("peer:", "").trim().to_string();
        peer_list.add_peer(peer_addr);
        return;
    }

    if let Ok(block) = serde_json::from_str::<Block>(&received) {
        blockchain.add_block(block);
    }
    else if let Ok(chain) = serde_json::from_str::<Vec<Block>>(&received) {
        blockchain.replace_chain(chain);
    }
}

pub fn send_block(block: &Block, address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let data = serde_json::to_string(block).unwrap();
    stream.write_all(data.as_bytes()).unwrap();
}

pub fn send_chain(chain: &Vec<Block>, address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let data = serde_json::to_string(chain).unwrap();
    stream.write_all(data.as_bytes()).unwrap();
}

pub fn send_peer(address: &str, peer_address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let msg = format!("peer:{}", peer_address);
    stream.write_all(msg.as_bytes()).unwrap();
}
