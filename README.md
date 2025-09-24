# 🧮 Limit Order Book Engine

A performance-focused trading engine built in Rust with a real-time React dashboard. It mirrors the core mechanics of modern exchanges using FIFO limit order matching and WebSocket-based data broadcasting.

---

## 📋 Table of Contents

- 🤖 [Introduction](#-introduction)  
- ⚙️ [Tech Stack](#-tech-stack)  
- 🔋 [Features](#-features)  
- 🤸 [Quick Start](#-quick-start)  
- 🕸️ [Snippets](#-snippets)  
- 🔗 [Links](#-links)  
- 🚀 [More](#-more)  
- 🚨 [Tutorial](#-tutorial)

---

## 🤖 Introduction

This project simulates a working limit order book, the foundation of modern trading platforms. It handles buy/sell order placement, cancellation, matching, and live trade display. The frontend syncs with the engine in real time using WebSockets and reflects all activity in a fully interactive UI.

Whether you're exploring Rust performance, real-time state management, or order book design, this is a hands-on sandbox for building trading logic from the ground up.

---

## ⚙️ Tech Stack

- 🦀 **Rust** – backend engine with Axum + Tokio
- 🧠 **React** – frontend dashboard (with TypeScript)
- 📡 **WebSockets** – real-time bidirectional updates
- ⚛️ **Recoil** – client state management
- 💨 **Tailwind CSS** – UI styling
- 📈 **Criterion** – benchmarking
- 🔥 **Flamegraph** – performance profiling
- 🧱 **(Planned)** Redis – persistence layer

---

## 🔋 Features

👉 FIFO-based limit order matching engine  
👉 Real-time trade feed via WebSockets  
👉 Order book synced across all clients  
👉 User-specific "My Orders" view with cancel actions  
👉 Stateless engine with clear separation of concerns  
👉 Performance profiling with Flamegraph (optional)  
👉 Modular frontend UI with Recoil-based sync

---

## 🤸 Quick Start

### 📦 Prerequisites

- Rust (v1.70+ recommended)
- Node.js + npm
- (Optional) Flamegraph: `cargo install flamegraph`

### 🧱 Installation

```bash
git clone https://github.com/tdkayy/limit-order-book
cd limit-order-book
