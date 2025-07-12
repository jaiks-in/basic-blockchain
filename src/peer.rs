use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PeerList {
    pub peers: Arc<Mutex<Vec<String>>>,
}

impl PeerList {
    pub fn new() -> Self {
        PeerList { peers: Arc::new(Mutex::new(vec![])) }
    }

    pub fn add_peer(&self, address: String) {
        let mut peers = self.peers.lock().unwrap();
        if !peers.contains(&address) {
            peers.push(address.clone());
            println!("✅ Peer added: {}", address);
        }
    }

    pub fn get_peers(&self) -> Vec<String> {
        self.peers.lock().unwrap().clone()
    }

    pub fn print_peers(&self) {
        for p in self.peers.lock().unwrap().iter() {
            println!(" - {}", p);
        }
    }
}
