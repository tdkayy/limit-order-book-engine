use crate::order::{Order, OrderSide};
use arrayvec::ArrayVec;
use std::collections::BTreeMap;

/// Each price level holds a queue of orders (FIFO)
pub type PriceLevel = ArrayVec<Order, 8>;

pub struct OrderBook {
    pub bids: BTreeMap<u64, PriceLevel>,
    pub asks: BTreeMap<u64, PriceLevel>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let book_side = match order.side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        let level = book_side.entry(order.price).or_insert_with(|| ArrayVec::new());

        if level.try_push(order.clone()).is_err() {
            #[cfg(debug_assertions)]
            println!("Overflowed price level at {}", order.price);
        }
    }

    pub fn cancel_order(&mut self, order_id: u64) -> bool {
        for book in [&mut self.bids, &mut self.asks] {
            let mut to_remove = None;
            for (price, level) in book.iter_mut() {
                if let Some(pos) = level.iter().position(|o| o.id == order_id) {
                    level.remove(pos);
                    if level.is_empty() {
                        to_remove = Some(*price);
                    }
                    return true;
                }
            }
            if let Some(price) = to_remove {
                book.remove(&price);
            }
        }
        false
    }

    pub fn get_best_bid(&self) -> Option<u64> {
        self.bids.keys().rev().next().copied()
    }

    pub fn get_best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }

    pub fn execute_order(&mut self, mut incoming: Order) -> Vec<(u64, u32)> {
        let mut fills = Vec::new();

        let book = match incoming.side {
            OrderSide::Buy => &mut self.asks,
            OrderSide::Sell => &mut self.bids,
        };

        let mut to_remove = Vec::new();

        for (&price, level) in book.iter_mut() {
            let match_possible = match incoming.side {
                OrderSide::Buy => incoming.price >= price,
                OrderSide::Sell => incoming.price <= price,
            };

            if !match_possible {
                break;
            }

            let mut i = 0;
            while i < level.len() && incoming.quantity > 0 {
                let resting = &mut level[i];
                let traded_qty = incoming.quantity.min(resting.quantity);
                incoming.quantity -= traded_qty;
                resting.quantity -= traded_qty;
                fills.push((resting.id, traded_qty));

                if resting.quantity == 0 {
                    level.remove(i);
                } else {
                    i += 1;
                }
            }

            if level.is_empty() {
                to_remove.push(price);
            }

            if incoming.quantity == 0 {
                break;
            }
        }

        for price in to_remove {
            book.remove(&price);
        }

        if incoming.quantity > 0 {
            self.add_order(incoming);
        }

        fills
    }
}
