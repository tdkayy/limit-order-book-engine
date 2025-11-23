# High-Performance Limit Order Book (Rust)

![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Performance](https://img.shields.io/badge/throughput-8.8M%20orders%2Fs-green?style=flat-square)

A low-latency, single-threaded matching engine written in Rust, capable of processing **8.8 million orders per second** on standard hardware. Designed for high-frequency trading (HFT) simulations, this engine implements strict Price-Time priority with $O(1)$ order cancellations.

## 🚀 Performance
Benchmarked on a MacBook Pro (M-Series):
- **Throughput:** ~8,830,000 orders/second
- **Latency:** Sub-microsecond execution time per order
- **Load:** Sustained stress test of 1,000,000 continuous order cycles (2M operations)

```text
🚀 Starting stress test with 1000000 orders...
✅ Matched 1000000 pairs of orders
⏱️ Time taken: 226.49ms
⚡ Throughput: 8830069 orders/second

graph TD
    A[API / WebSocket] -->|JSON| B(Order Gateway)
    B -->|Struct| C{Matching Engine}
    C -->|Limit Order| D[BTreeMap: Price Levels]
    C -->|Cancel| E[HashMap: Order Index]
    D -->|Match Found| F[Trade Execution]
    D -->|No Match| G[Add to Book]
    F -->|Event| H[Broadcast to Clients]



