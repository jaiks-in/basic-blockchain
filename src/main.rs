mod blockchain;
mod wallet;
mod network;
mod peer;

use crate::blockchain::{Blockchain, Transaction};
use crate::wallet::Wallet;
use crate::network::{start_server, send_peer, send_chain};
use crate::peer::PeerList;
use std::thread;
use std::time::Duration;

fn main() {
    let mut blockchain = Blockchain::new(3);
    let peer_list = PeerList::new();
    let my_addr = "127.0.0.1:6000";

    peer_list.add_peer(my_addr.to_string());
    let pl_clone = peer_list.clone();
    let bc_clone = blockchain.clone();

    thread::spawn(move || {
        start_server(bc_clone, pl_clone, my_addr);
    });

    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();

    let mut tx = Transaction {
        sender: format!("{:?}", wallet1.public_key),
        receiver: format!("{:?}", wallet2.public_key),
        amount: 50,
        signature: None,
    };
    tx.sign_transaction(&wallet1.private_key);
    blockchain.add_transaction(tx);

    blockchain.mine_pending_transactions(format!("{:?}", wallet2.public_key), my_addr);
    for peer in peer_list.get_peers() {
        send_chain(&blockchain.chain, &peer);
    }

    thread::sleep(Duration::from_secs(2));
    blockchain.print_chain();
    peer_list.print_peers();
}
