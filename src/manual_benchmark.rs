use chrono::Utc;
use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;
use std::time::Instant;

fn main() {
    let mut book = OrderBook::new();
    let n = 1_000_000; // 1 Million Orders

    println!("🚀 Starting stress test with {} orders...", n);

    let start = Instant::now();

    for i in 0..n {
        // 1. Add a SELL order at price 100
        let sell_order = Order {
            id: i * 2, // Even IDs for sells
            side: OrderSide::Sell,
            price: 100,
            quantity: 1,
            timestamp: Utc::now().naive_utc(),
        };
        book.add_order(sell_order);

        // 2. Add a BUY order at price 100 (Matches immediately)
        let buy_order = Order {
            id: (i * 2) + 1, // Odd IDs for buys
            side: OrderSide::Buy,
            price: 100,
            quantity: 1,
            timestamp: Utc::now().naive_utc(),
        };
        book.execute_order(buy_order);
    }

    let duration = start.elapsed();

    println!("✅ Matched {} pairs of orders", n);
    println!("⏱️ Time taken: {:?}", duration);
    println!("⚡ Throughput: {:.0} orders/second", (n as f64 * 2.0) / duration.as_secs_f64());
    
    // Sanity check: The book should be empty if everything matched
    println!("🔧 Remaining best ask: {:?}", book.get_best_ask());
    println!("🔧 Remaining best bid: {:?}", book.get_best_bid());
}