use limit_order_book_engine::{Order, OrderBook, OrderSide};
use chrono::NaiveDateTime;

fn main() {
    let mut ob = OrderBook::new();
    let dummy_time = NaiveDateTime::from_timestamp(0, 0);

    for id in 0..100_000 {
        let order = Order {
            id,
            side: OrderSide::Buy,
            price: 100,
            quantity: 10,
            timestamp: dummy_time,
        };
        ob.add_order(order);
    }
}
