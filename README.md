# High-Performance Limit Order Book (Rust)

![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Performance](https://img.shields.io/badge/throughput-8.8M%20orders%2Fs-green?style=flat-square)

A low-latency, single-threaded matching engine written in Rust, capable of processing **8.8 million orders per second** on standard hardware. Designed for high-frequency trading (HFT) simulations, this engine implements strict Price-Time priority with $O(1)$ order cancellations.

## Performance

Benchmarked on a MacBook Pro (M-Series):
* **Throughput:** ~8,830,000 orders/second
* **Latency:** Sub-microsecond execution time per order
* **Load:** Sustained stress test of 1,000,000 continuous order cycles (2M operations)

```text
 Starting stress test with 1000000 orders...
 Matched 1000000 pairs of orders
 Time taken: 226.49ms
 Throughput: 8830069 orders/second
```

## Architecture

The engine optimizes for memory locality and algorithmic efficiency using a hybrid data structure approach:
* Core Data Structures

Price Levels (BTreeMap<u64, VecDeque<Order>>):
* Uses a B-Tree to keep price levels sorted (Bids descending, Asks ascending).
* Uses VecDeque for FIFO (First-In-First-Out) order queues at each price level, reducing memory reallocation overhead during matches.

Order Index (HashMap<u64, u64>):
* Maps OrderID -> Price to enable O(1) constant-time cancellations.
* Avoids the typical O(N) scan required by naive implementations when removing orders

```text
graph TD
    A[API / WebSocket] -->|JSON| B(Order Gateway)
    B -->|Struct| C{Matching Engine}
    C -->|Limit Order| D[BTreeMap: Price Levels]
    C -->|Cancel| E[HashMap: Order Index]
    D -->|Match Found| F[Trade Execution]
    D -->|No Match| G[Add to Book]
    F -->|Event| H[Broadcast to Clients]
```

## Tech Stack
* Core Logic: Rust (Safe, Zero-Cost Abstractions)
* Server: Axum (High-performance Async Web Framework)
* Runtime: Tokio (Asynchronous I/O)
* Benchmarking: Custom Criterion-style micro-benchmarking

## Usage
1. Run the Engine (API Server)
Starts the WebSocket and REST API server.
```text
cargo run --release
```

2. Run the Benchmark
Executes the stress test script to verify throughput.
```text
cargo run --release --bin manual_benchmark
```

3. Run Unit Tests
Verifies matching logic, partial fills, and price-time priority.
```text
cargo test
```

## Key Features
* Price-Time Priority: Orders are matched strictly based on best price, then earliest timestamp.
* Partial Fills: Handles orders larger than the liquidity at the top of the book correctly.
* Real-Time Data: Exposes WebSocket endpoints for live order book updates and trade feeds.
* Memory Safety: Leverages Rust's ownership model to ensure thread safety without garbage collection pauses.

## Future Improvements
* Lock-Free Data Structures: Migrating from Mutex<OrderBook> to Atomic-based structures (e.g., Crossbeam) to reduce contention.
* SPSC Queue: Implementing a Single-Producer-Single-Consumer ring buffer for handling incoming network packets.
