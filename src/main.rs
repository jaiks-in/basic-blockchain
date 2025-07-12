mod blockchain;
use blockchain::{Blockchain, Transaction, Wallet};
mod network;
use network::{start_server,send_block};
use std::thread;
use std::time::Duration;
fn main() {
    let mut blockchain = Blockchain::new(4);

    // Wallets create
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    let server_blockchain=blockchain.clone();
    thread::spawn(move||{
        start_server(server_blockchain, "127.0.0.1:6000");
    });
    thread::sleep(Duration::from_secs(1));
    // Transaction banaye
    let mut tx1 = Transaction {
        sender: format!("{:?}", wallet1.public_key.to_encoded_point(false)),
        receiver: format!("{:?}", wallet2.public_key.to_encoded_point(false)),
        amount: 50,
        signature: None,
    };

    // Sign transaction with wallet1's private key
    tx1.sign_transaction(&wallet1.private_key);

    // Verify transaction
    if tx1.is_valid(&wallet1.public_key) {
        println!("✅ Transaction verified.");
        blockchain.add_transaction(tx1);
    } else {
        println!("❌ Invalid transaction.");
    }

    // Mining
    blockchain.mine_pending_transactions("MinerBhai".to_string());

    // Chain print
    blockchain.print_chain();

    // Validity check
    println!("Blockchain valid? {}", blockchain.is_valid());
}
