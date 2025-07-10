use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;
use chrono::NaiveDateTime;
use std::time::Instant;

fn main() {
    let mut book = OrderBook::new();

    let now = NaiveDateTime::from_timestamp(0, 0);

    // Insert 1000 ASK orders: Price = 1000 to 1999, Quantity = 1
    for i in 0..1000 {
        let ask = Order {
            id: i,
            side: OrderSide::Sell,
            price: 1000 + i,
            quantity: 1,
            timestamp: now,
        };
        book.add_order(ask);
    }

    // One large BUY order to sweep the book
    let big_buy = Order {
        id: 9999,
        side: OrderSide::Buy,
        price: 2000,  // Aggressive price
        quantity: 1000,
        timestamp: now,
    };

    let start = Instant::now();
    let fills = book.execute_order(big_buy);
    let elapsed = start.elapsed();

    // Verify result
    println!("✅ Matched {} orders in {:?}", fills.len(), elapsed);
    println!("🔧 Remaining best ask: {:?}", book.get_best_ask());
    println!("🔧 Remaining best bid: {:?}", book.get_best_bid());

    // Optional: Uncomment for detailed fill output
    // for (id, qty) in fills {
    //     println!("Filled order ID: {}, Qty: {}", id, qty);
    // }
}
