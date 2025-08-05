📋 Table of Contents

🤖 Introduction

⚙️ Tech Stack

🔋 Features

🤸 Quick Start

🕸️ Snippets

🔗 Links

🚀 More


🚨 Tutorial

🤖 Introduction
The Limit Order Book Engine is a performance-focused trading simulation system built in Rust with a React frontend. It mirrors the architecture of real-world exchanges, allowing users to place buy/sell orders, observe a live order book, and view trade history in real time.

This project is perfect for exploring real-time systems, concurrency in Rust, and WebSocket-based client syncing — all through a fully interactive trading UI.


⚙️ Tech Stack
- Rust (Axum, Tokio, Serde)

- React + TypeScript + Tailwind CSS

- WebSockets (custom broadcast system)

- Recoil for state management

- Criterion & Flamegraph (performance profiling)

- Planned: Redis for data persistence


🔋 Features
👉 FIFO-based buy/sell order matching

👉 Live trade feed over WebSockets

👉 Real-time order book with client sync

👉 "My Orders" dashboard with cancel functionality

👉 Stateless backend with async broadcast

👉 Type-safe data models and event-driven logic

👉 Simple setup — no auth, no DB, just code


🤸 Quick Start

Follow these instructions to help set up on your loacl machine

📦 Prerequisites

- Rust

- Node.js

- cargo install flamegraph (optional)


🧱 Clone & Install

git clone https://github.com/tdkayy/limit-order-book
cd limit-order-book


Run backend:
cd backend
cargo run


Run frontend:
cd frontend
npm install
npm run dev


🕸️ Snippets
- src/hooks/useOrderBookSocket.ts — live WebSocket state sync

- api_server.rs — order routing & matching logic

- components/OrderBook.tsx — frontend LOB renderer

- flamegraph.svg — optional CPU profiling report

- types/index.ts — shared data model interfaces


🔗 Links
- 🔗 GitHub Repo

- 🌐 Live Demo – Coming Soon


🚨 Tutorial
Want a step-by-step breakdown of how this was built?
📺 A full written or video walkthrough may be published soon.
Follow @tdkayy for updates!
