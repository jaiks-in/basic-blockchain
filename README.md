# 🔗 Rust Blockchain P2P Prototype 🚀

A simple blockchain implementation in Rust with basic Proof-of-Work (PoW) mining and TCP-based peer-to-peer (P2P) networking.  
This project demonstrates how blocks are created, mined, transferred, and validated between nodes using TCP sockets.

---

## 📌 Features

- ✅ Basic Blockchain with Blocks containing:
  - Timestamp
  - Data
  - Previous Hash
  - Nonce
  - Hash

- ✅ Proof-of-Work Mining Algorithm  
  (Adjustable difficulty level — finds a hash starting with N zeros)

- ✅ TCP Server-Client Networking
  - Accepts incoming block data from peer nodes
  - Sends block data to connected peers

- ✅ JSON Serialization & Deserialization using `serde_json`

- ✅ Multi-threaded Connection Handling with Rust's `std::thread`

---

## 📦 Project Structure

```

📂 src
┣ 📜 main.rs            // Entry point: runs server and triggers mining
┣ 📜 blockchain.rs      // Blockchain & Block definitions + logic
┣ 📜 network.rs         // TCP networking functions (server, client)
┣ 📜 lib.rs             // (if using as a library)
📄 Cargo.toml

````

---

## ⚙️ How It Works

1️⃣ **Start the blockchain server:**

```bash
cargo run
````

2️⃣ Server listens for incoming TCP connections on the specified address.

3️⃣ Mines a new block with Proof-of-Work.

4️⃣ New blocks can be sent to other peers via TCP connection.

5️⃣ Receives blocks from peers and adds them to its chain after basic validation.

---

## 📡 TCP API Overview

* `start_server(blockchain, address)`
  → Starts TCP server to accept blocks from other nodes.

* `send_block(block, address)`
  → Sends a mined block to another peer node via TCP.

---

## 📦 Dependencies

* `serde`
* `serde_json`
* `chrono`
* `sha2`
* `std::net` (for TcpListener & TcpStream)
* `std::thread`

Add them via Cargo.toml.

---

## 📸 Example

```bash
✅ Block mined: 00000f7a28e7dbce... (nonce: 4821)
✅ Block added! Current Chain: [Block#1, Block#2, ...]
Received:
{"timestamp": "2025-07-09 13:00:12", "data": "New Block!", "prev_hash": "...", ...}
✅ Block added! Current Chain: [Block#1, Block#2, Block#3]
```

---

## 📚 What I Learned

* Rust struct, traits & ownership
* SHA-256 hashing in Rust
* Proof-of-Work concept implementation
* TCP client-server communication
* Multithreading with `std::thread`
* JSON serialization / deserialization with `serde_json`

---

## 📈 Future Improvements

* ✅ Transaction struct and transaction pool
* ✅ Block validation logic
* ✅ Persistent storage with `sled` or `rocksdb`
* ✅ WebSocket or gRPC-based networking
* ✅ Consensus mechanism (PoS, PoW, PBFT)

---

## 👨‍💻 Author

**Jai Sharma**
Rust Developer | Blockchain Enthusiast

---

## ⭐️ If you found this interesting, give it a ⭐️ and fork the repo!

Let’s build decentralized systems together 🚀


