use criterion::{black_box, criterion_group, criterion_main, Criterion};
use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;
use chrono::Utc;

fn bench_add_order(c: &mut Criterion) {
    let timestamp = Utc::now().naive_utc();

    c.bench_function("order_book_throughput", |b| {
        b.iter(|| {
            // Measure "end-to-end" processing
            let mut ob = OrderBook::new();
            
            // Batch of 10,000 orders
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

// Group the benchmarks
criterion_group!(benches, bench_add_order);
criterion_main!(benches);