# Avantix

**A deterministic Rust limit order book and exchange-simulation project.**

Avantix is a systems-engineering project for studying order matching, price-level data structures, asynchronous API boundaries, testing, and performance measurement. It is intentionally scoped as an educational exchange simulator rather than presented as a production trading venue.

> **Status:** pre-v0.1 and under active correctness/integration work. The repository is public so the design decisions, limitations, and progress remain inspectable.

## Purpose

Avantix exists to provide a small, understandable environment for exploring:

- limit-order-book mechanics;
- price and time priority;
- full and partial fills;
- cancellation and order indexing;
- deterministic event processing;
- REST and WebSocket integration with a Rust core;
- correctness testing, fuzzing, profiling, and reproducible benchmarks.

The goal is not to imitate every component of a commercial exchange. The goal is to build a narrow system whose behaviour can be specified, tested, measured, and explained.

## Current implementation

The repository currently contains:

- an in-memory Rust order-book library;
- bid and ask price levels stored in `BTreeMap<u64, VecDeque<Order>>`;
- FIFO queues within each individual price level;
- order insertion, cancellation, best-bid/best-ask retrieval, and library-level matching with partial fills;
- an Axum/Tokio HTTP and WebSocket server;
- endpoints for submitting and cancelling orders and reading book state;
- unit tests for selected matching scenarios;
- a Criterion order-insertion microbenchmark;
- a separate synthetic matching stress test;
- a `cargo-fuzz` target for generated order inputs;
- Docker packaging and GitHub Actions checks for build, tests, and formatting.

## Architecture today

```text
HTTP / WebSocket clients
          |
          v
      Axum server
          |
          v
 Arc<Mutex<OrderBook>>
          |
          +----> BTreeMap bid levels
          +----> BTreeMap ask levels
          +----> order-location index
```

This shared-state server design is deliberately simple while the matching rules are stabilised. A future iteration will move the engine behind a single-owner command-processing task so order sequencing and backpressure can be reasoned about more explicitly.

## Data-structure choices

### Ordered price levels

`BTreeMap` keeps price levels ordered and supports direct retrieval of the best ask and best bid. It is easier to inspect and reason about than a more specialised structure while the project is focused on correctness.

### FIFO within a price level

Each price level uses a `VecDeque<Order>`. New resting orders are appended to the back, and matching consumes orders from the front.

### Cancellation index

A `HashMap` maps an order ID to its price. This avoids scanning every price level, but cancellation still performs a linear search within the identified level. **Avantix does not currently claim constant-time cancellation.**

## Correctness model

The intended matching rules for v0.1 are:

1. A buy order matches the lowest eligible ask first.
2. A sell order matches the highest eligible bid first.
3. Orders at the same price execute FIFO.
4. A trade uses the resting order's price.
5. Partial resting orders retain their queue position.
6. Any unfilled limit-order quantity rests on the book.
7. Empty price levels are removed.
8. Cancelled or fully filled orders are removed from the active-order index.
9. Processing an accepted command must not leave a crossed book.

Each rule will be represented by focused tests before v0.1 is tagged.

## Current pre-v0.1 limitations

The following items are known and are part of the v0.1 work rather than hidden behind performance claims:

- sell-side best-price traversal requires correction and mirrored tests;
- the order-submission API currently inserts an order directly instead of routing it through matching execution;
- the WebSocket event route does not yet publish a dedicated structured `Trade` event;
- cancellation is linear within one price level;
- the current fuzz target exercises a single generated insertion rather than command sequences and invariants;
- the benchmark suite does not represent network, JSON, persistence, or production exchange throughput.

## Running the project

### Requirements

- Rust stable toolchain
- Cargo

### Build and test

```bash
cargo build
cargo test
cargo fmt -- --check
```

### Start the API server

```bash
cargo run --release
```

The server binds to `127.0.0.1:4000`.

### Current routes

| Method | Route | Purpose |
|---|---|---|
| `GET` | `/` | Health response |
| `POST` | `/api/orders` | Submit an order to the current API layer |
| `POST` | `/api/orders/cancel` | Cancel an active order by ID |
| `GET` | `/api/orders/all` | Return all resting orders |
| `GET` | `/api/orderbook` | Return the current order-book state |
| `GET` | `/ws/orderbook` | Stream periodic order-book snapshots |
| `GET` | `/ws/trades` | Stream current order-related events; dedicated trade events are planned for v0.1 |

### Example order

```bash
curl -X POST http://127.0.0.1:4000/api/orders \
  -H 'Content-Type: application/json' \
  -d '{"side":"buy","price":100,"quantity":10}'
```

### Example cancellation

```bash
curl -X POST http://127.0.0.1:4000/api/orders/cancel \
  -H 'Content-Type: application/json' \
  -d '{"order_id":123}'
```

## Benchmarks

The repository currently has two different benchmark paths:

```bash
# Criterion microbenchmark for bulk insertion
cargo bench

# Synthetic in-memory matching stress test
cargo run --release --bin manual_benchmark
```

No production-capacity or tail-latency claim is made from these tests. Before publishing new figures, the benchmark documentation will state the hardware, workload, warm-up, operation definition, included/excluded layers, and latency distribution.

## Fuzzing

Install `cargo-fuzz` and run:

```bash
cargo +nightly fuzz run fuzz_target_1
```

The current target is intentionally described as an insertion crash test. Sequence-based fuzzing and invariant checks are planned for v0.1.

## Docker

```bash
docker build -t avantix .
docker run --rm -p 4000:4000 avantix
```

## v0.1 acceptance criteria

- [ ] Correct and test best-price traversal on both sides of the book
- [ ] Cover FIFO, full fill, partial fill, multi-level fill, cancellation, and invalid input
- [ ] Route API submissions through matching execution
- [ ] Add structured trade events
- [ ] Add book invariants and command-sequence fuzzing
- [ ] Add Clippy to CI and treat warnings as errors
- [ ] Replace headline benchmarks with documented scenario-based results
- [ ] Remove stale artefacts and publish a clean release tag

## Scope deliberately excluded from v0.1

Avantix does not currently claim to implement FIX, kernel bypass, lock-free queues, distributed consensus, custom allocation, zero-copy network parsing, or production exchange guarantees. Those technologies will only be introduced if a measured constraint justifies them.

## Licence

No licence has been selected yet. Until one is added, the source is publicly viewable but normal copyright restrictions apply.
