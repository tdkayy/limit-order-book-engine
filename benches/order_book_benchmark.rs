use criterion::{black_box, criterion_group, criterion_main, Criterion};
use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;
use chrono::Utc;

fn bench_add_order(c: &mut Criterion) {
    let timestamp = Utc::now().naive_utc();

    c.bench_function("add_order x 10,000", |b| {
        b.iter(|| {
            let mut ob = OrderBook::new();
            for i in 0..10_000 {
                let order = Order {
                    id: i,
                    side: OrderSide::Buy,
                    price: 1000,
                    quantity: 10,
                    timestamp,
                };
                ob.add_order(black_box(order));
            }
        });
    });
}

fn bench_execute_order(c: &mut Criterion) {
    let timestamp = Utc::now().naive_utc();

    c.bench_function("execute_order x 10,000", |b| {
        b.iter(|| {
            let mut ob = OrderBook::new();

            // Add sell-side liquidity to be matched
            for i in 0..10_000 {
                let ask = Order {
                    id: i,
                    side: OrderSide::Sell,
                    price: 1000,
                    quantity: 10,
                    timestamp,
                };
                ob.add_order(ask);
            }

            // Now match against them
            let buy = Order {
                id: 10_001,
                side: OrderSide::Buy,
                price: 1000,
                quantity: 100_000, // Will match all 10k sell orders
                timestamp,
            };

            black_box(ob.execute_order(buy));
        });
    });
}

criterion_group!(benches, bench_add_order, bench_execute_order);
criterion_main!(benches);
