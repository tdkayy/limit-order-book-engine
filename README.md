# Avantix

[![Rust CI](https://github.com/tdkayy/limit-order-book-engine/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/tdkayy/limit-order-book-engine/actions/workflows/ci.yml)
![Status](https://img.shields.io/badge/status-pre--v0.1-orange)
![Language](https://img.shields.io/badge/language-Rust-black)

**Avantix is a Rust limit-order-book engine and exchange simulator built to explore matching correctness, order-book data structures, asynchronous APIs, testing, and performance measurement.**

> **Status:** pre-v0.1 correctness hardening. Avantix is an engineering project, not a production exchange or a claim of production trading-system performance.

## Purpose

Matching engines sit at the intersection of algorithms, systems design, financial-market rules, and performance engineering. Avantix provides a focused environment for studying:

- price and time priority;
- full and partial fills;
- cancellation and active-order indexing;
- deterministic state transitions;
- REST and WebSocket integration;
- correctness testing, fuzzing, profiling, and benchmarking;
- the trade-offs between simple and specialised data structures.

The first release prioritises **correctness, reproducibility, and defensible design decisions** over headline throughput figures or premature distributed architecture.

## Current capabilities

- In-memory bid and ask books
- Ordered price levels using `BTreeMap`
- FIFO queues at each price using `VecDeque`
- Limit-order insertion, partial fills, cancellation, and best-price retrieval
- Axum REST API and WebSocket streams
- Tokio-based shared application state
- Unit tests for core matching behaviour
- Criterion microbenchmarks and a synthetic release-mode benchmark
- `cargo-fuzz` target
- Docker packaging
- GitHub Actions checks for build, tests, and formatting

## Architecture

```text
HTTP / WebSocket clients
          |
          v
      Axum server
          |
          v
 Arc<Mutex<OrderBook>>
          |
          +----> BTreeMap<u64, VecDeque<Order>> bids
          +----> BTreeMap<u64, VecDeque<Order>> asks
          +----> HashMap<OrderId, Price> location index
          |
          +----> broadcast channel for API events
```

This shared-state design is deliberately simple while the matching contract is stabilised. A later iteration will place the engine behind a single-owner command-processing task so sequencing, backpressure, and event publication can be handled explicitly.

## v0.1 correctness contract

Avantix v0.1 will guarantee that:

1. A buy order matches the lowest eligible ask first.
2. A sell order matches the highest eligible bid first.
3. Resting orders at the same price execute FIFO.
4. Trades execute at the resting order's price.
5. Partially filled resting orders retain queue position.
6. Unfilled limit-order quantity rests on the book.
7. Empty price levels are removed.
8. Cancelled and fully filled orders leave the active-order index.
9. Accepted commands do not leave the book crossed.
10. Invalid and duplicate orders are rejected explicitly.

Each rule will be represented by focused tests before `v0.1.0` is tagged.

## Data-structure choices

### Price levels

Bids and asks use separate `BTreeMap<u64, VecDeque<Order>>` structures. `BTreeMap` keeps price levels ordered and makes best-price retrieval straightforward. It is being used as a readable correctness-first baseline rather than presented as the fastest possible structure.

### Time priority

Each price level uses a `VecDeque<Order>`. New resting orders are appended to the back, while matching consumes orders from the front.

### Cancellation

A `HashMap<OrderId, Price>` identifies the price level containing an active order. The current implementation then scans within that level, so cancellation is approximately:

```text
O(log P + L)
```

where `P` is the number of price levels and `L` is the number of orders at the identified level. Avantix does **not** currently claim constant-time cancellation.

## API

The server listens on `127.0.0.1:4000`.

| Method | Route | Purpose |
|---|---|---|
| `GET` | `/` | Health check |
| `POST` | `/api/orders` | Submit an order |
| `POST` | `/api/orders/cancel` | Cancel an active order |
| `GET` | `/api/orders/all` | Return all resting orders |
| `GET` | `/api/orderbook` | Return the current book |
| `GET` | `/ws/orderbook` | Stream periodic book snapshots |
| `GET` | `/ws/trades` | Stream API events; dedicated trade events are planned for v0.1 |

### Submit an order

```bash
curl -X POST http://127.0.0.1:4000/api/orders \
  -H "Content-Type: application/json" \
  -d '{"side":"buy","price":101,"quantity":10}'
```

### Cancel an order

```bash
curl -X POST http://127.0.0.1:4000/api/orders/cancel \
  -H "Content-Type: application/json" \
  -d '{"order_id":123456789}'
```

## Run locally

### Requirements

- Rust stable toolchain
- Cargo

### Build and test

```bash
cargo build
cargo test
cargo fmt -- --check
```

### Start the API

```bash
cargo run --release
```

### Run benchmarks

```bash
cargo bench
cargo run --release --bin manual_benchmark
```

### Docker

```bash
docker build -t avantix .
docker run --rm -p 4000:4000 avantix
```

> The server currently binds to `127.0.0.1`; container networking will be corrected before Docker-based API access is treated as a supported v0.1 workflow.

## Benchmarking policy

The current Criterion benchmark measures batched same-price order insertion. The manual benchmark measures a synthetic loop that repeatedly adds a resting sell and executes a crossing buy.

These are development workloads, not production exchange benchmarks. They exclude network transport, JSON processing, persistence, realistic distributions, multiple instruments, backpressure, and end-to-end tail latency.

Avantix will not publish headline throughput, p99 latency, zero-allocation, or production-scale claims until the relevant methodology measures those properties directly and can be reproduced.

## Work required for v0.1

- [ ] Correct and test sell-side best-price traversal
- [ ] Route API submissions through matching execution
- [ ] Introduce a structured `Trade` event
- [ ] Separate book updates from execution events
- [ ] Add full-fill, partial-fill, multi-level, FIFO, cancellation, and invalid-input tests
- [ ] Reject duplicate order IDs
- [ ] Replace the single-insertion fuzz target with command sequences and invariants
- [ ] Split benchmarks into insertion, matching, cancellation, and mixed workloads
- [ ] Add Clippy to CI
- [ ] Align package, Docker, repository, and public naming around Avantix
- [ ] Tag `v0.1.0` only after the correctness contract is covered

## Roadmap

### v0.1 — Correct matching core

Bidirectional price-time priority, structured executions, API integration, validation, comprehensive tests, and reproducible benchmark documentation.

### v0.2 — Determinism and recovery

Sequence numbers, append-only event logging, deterministic replay, snapshots, and recovery tests.

### v0.3 — Stronger verification

Property-based testing, command-sequence fuzzing, explicit invariants, and model-based comparison with a slower reference implementation.

### v0.4 — Measured optimisation

Realistic mixed-workload benchmarks, profiling-guided optimisation, cancellation data-structure experiments, and a single-owner engine task with bounded queues.

## Project structure

```text
.
├── .github/workflows/ci.yml
├── benches/order_book_benchmark.rs
├── fuzz/fuzz_targets/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── manual_benchmark.rs
│   ├── order.rs
│   └── order_book.rs
├── Cargo.toml
└── Dockerfile
```

## Scope

Avantix is not currently a regulated exchange, brokerage platform, distributed matching cluster, FIX gateway, kernel-bypass stack, or zero-allocation lock-free engine. Adding those technologies before the core is correct and measurable would make the project larger, not better.

## Author

Built by [Tolu Adesanya](https://github.com/tdkayy) as a systems-engineering project focused on financial-market infrastructure, correctness, and performance reasoning.