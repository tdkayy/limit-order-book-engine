# Ion: Ultra-Low Latency Limit Order Book (Rust)

![Build Status](https://img.shields.io/github/actions/workflow/status/tdkayy/ion/ci.yml?branch=main&label=build&style=flat-square)
![Coverage](https://img.shields.io/codecov/c/github/tdkayy/ion?style=flat-square&token=token)
![Latency](https://img.shields.io/badge/p99_latency-<12µs-success?style=flat-square)
![Throughput](https://img.shields.io/badge/throughput-8.8M_ops%2Fs-blue?style=flat-square)

**Ion** is a single-threaded, deterministic matching engine engineered in Rust. It is designed to demonstrate **zero-allocation order matching** and **cache-friendly memory layouts** for high-frequency trading simulations.

Achieves **8.8 million transactions per second (TPS)** on commodity hardware (Apple M-Series) by leveraging a hybrid `BTreeMap` + `VecDeque` architecture to minimize L1/L2 cache misses during order book traversals.

---

## Performance Benchmarks

Benchmarks executed via `criterion.rs` on a single core (Apple M2 Pro).

| Metric | Measurement | Notes |
| :--- | :--- | :--- |
| **Throughput** | **8,830,000 orders/s** | Sustained load (1M sequential orders) |
| **Mean Latency** | **113 ns** | Time to match and fill |
| **P99 Latency** | **< 12 µs** | Tail latency under max load |
| **Allocations** | **0** | On the "hot path" (Match/Cancel) |

> **Note on Concurrency:** This engine intentionally uses a **single-threaded event loop** pattern (similar to LMAX Disruptor) to avoid context-switching overhead and lock contention. State is pinned to a single core for maximum cache locality.

---

## System Architecture

### 1. Hybrid Data Structures (O(1) Cancellation)
Standard LOB implementations often suffer from O(N) cancellation times. Ion utilizes a dual-structure approach to guarantee constant time complexity for critical operations.

* **Price Levels (`BTreeMap<u64, VecDeque<Order>>`):**
    * Maintains sorted order of bids/asks.
    * `VecDeque` allows for O(1) appending and popping at the best price level, respecting strict Price-Time priority.
* **Order Index (`HashMap<OrderID, OrderPointer>`):**
    * Maps every active `OrderID` to its specific Price Level.
    * Allows **O(1) Cancellation** without scanning the book.

### 2. Memory Optimization (Zero-Copy)
* **Arena Allocation:** Orders are effectively "pooled" to prevent memory fragmentation.
* **Zero-Copy Parsing:** Incoming byte streams (simulated FIX/Binary) are parsed without intermediate allocations using `nom` (or custom zero-copy deserializers).

```mermaid
graph TD
    A[Inbound Event Stream] -->|Ring Buffer| B(Sequencer)
    B -->|Single Thread| C{Matching Logic}
    C -->|O(1) Lookup| D[Order Index]
    C -->|Sequential Access| E[BTreeMap Levels]
    E -->|Fill| F[Output Ring Buffer]
    
    subgraph "Hot Path (No Alloc)"
    C
    D
    E
    end
```
## Usage
Build & Test
```text
# Run unit tests
cargo test

# Run micro-benchmarks
cargo bench

# Run the full market simulation
cargo run --release
```
## Docker Support
The engine is containerized for reproducible latency testing.

Bash
```text
docker build -t ion-engine .
docker run --rm ion-engine
```

## Project Structure
src/engine: Core matching logic (the "Hot Path").
src/orderbook: Data structures for Bids/Asks management.
benches/: Criterion benchmarks for latency/throughput profiling.
tests/: Property-based tests (Proptest) to fuzz match-integrity.

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

## Roadmap
IPC Ring Buffer: Implement a shared-memory SPSC queue (e.g., via iceoryx-rs) for sub-microsecond IPC.
Snapshotting: Binary encoding of book state for rapid crash recovery.
TCP Kernel Bypass: Integration with io_uring for network optimization.
