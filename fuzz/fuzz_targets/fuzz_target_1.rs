#![no_main]
use libfuzzer_sys::fuzz_target;
use limit_order_book_engine::{OrderBook, Order, OrderSide};
use chrono::Utc;

fuzz_target!(|data: (u64, u32, u64, bool)| {
    let (price, quantity, order_id, is_buy) = data;

    // Safety check: Don't process empty orders
    if price == 0 || quantity == 0 {
        return;
    }

    let mut book = OrderBook::new();
    let side = if is_buy { OrderSide::Buy } else { OrderSide::Sell };

    let order = Order {
        id: order_id,
        side,
        price,
        quantity,
        timestamp: Utc::now().naive_utc(),
    };

    // The crash test
    book.add_order(order);
});