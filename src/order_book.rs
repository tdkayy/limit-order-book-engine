use std::collections::{BTreeMap, VecDeque};
use crate::order::{Order, OrderSide};
use std::time::Instant;
use chrono::Utc;
use arrayvec::ArrayVec;

/// Each price level holds a queue of orders (FIFO)
pub type PriceLevel = ArrayVec<Order, 8>;

pub struct OrderBook {
    pub bids: BTreeMap<u64, PriceLevel>, // Sorted descending
    pub asks: BTreeMap<u64, PriceLevel>, // Sorted ascending
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
        
    pub fn add_order(&mut self, order: Order) {
        let start = Instant::now();
    
        let book = match order.side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };
    
        let level = book.entry(order.price)
            .or_insert_with(ArrayVec::new);
    
        if level.try_push(order).is_err() {
            #[cfg(debug_assertions)]
            println!("Overflowed price level at {}", order.price);
            // You could fall back to a Vec or skip if desired
        }
    
        let duration = start.elapsed();
        #[cfg(debug_assertions)]
        println!("Insertion took: {:?}", duration);
    }
                    
    fn get_book_mut(&mut self, side: OrderSide) -> &mut BTreeMap<u64, PriceLevel> {
        match side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        }
    }
    
    fn insert_into_price_level(&mut self, book: &mut BTreeMap<u64, PriceLevel>, order: Order) {
        let level = book.entry(order.price)
            .or_insert_with(ArrayVec::new);
    
        if level.try_push(order).is_err() {
            #[cfg(debug_assertions)]
            println!("Overflowed price level at {}", order.price);
        }
    }
        

    pub fn cancel_order(&mut self, order_id: u64) -> bool {
        for book in [&mut self.bids, &mut self.asks] {
            for (price, queue) in book.iter_mut() {
                if let Some(pos) = queue.iter().position(|o| o.id == order_id) {
                    queue.remove(pos);
                    #[cfg(debug_assertions)]
                    println!("Cancelled order with ID: {}", order_id);
                    
                    if queue.is_empty() {
                        let price_to_remove = *price;
                        book.remove(&price_to_remove);
                    }

                    return true;
                }
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
    
        let mut levels_to_remove = vec![];
    
        for (&price, queue) in book.iter_mut() {
            // Check if price is matchable
            let match_possible = match incoming.side {
                OrderSide::Buy => incoming.price >= price,
                OrderSide::Sell => incoming.price <= price,
            };
    
            if !match_possible {
                break;
            }
    
            while let Some(mut resting) = queue.first_mut() {
                let traded_qty = incoming.quantity.min(resting.quantity);
                incoming.quantity -= traded_qty;
                resting.quantity -= traded_qty;
    
                fills.push((resting.id, traded_qty));
    
                if resting.quantity == 0 {
                    queue.remove(0);
                }
    
                if incoming.quantity == 0 {
                    break;
                }
            }
    
            if queue.is_empty() {
                levels_to_remove.push(price);
            }
    
            if incoming.quantity == 0 {
                break;
            }
        }
    
        for price in levels_to_remove {
            book.remove(&price);
        }
    
        if incoming.quantity > 0 {
            self.add_order(incoming);
        }
    
        fills
    }    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::{Order, OrderSide};
    use chrono::Utc;

    #[test]
    fn test_add_and_cancel_order() {
        let mut ob = OrderBook::new();

        let order = Order {
            id: 1,
            side: OrderSide::Buy,
            price: 10000, // 100.00 in cents
            quantity: 10,
            timestamp: Utc::now().naive_utc(),
        };

        ob.add_order(order.clone());
        assert_eq!(ob.get_best_bid(), Some(10000));

        let removed = ob.cancel_order(1);
        assert!(removed);
        assert_eq!(ob.get_best_bid(), None);
    }
}
#[test]
fn test_order_matching() {
    let mut ob = OrderBook::new();

    let ask = Order {
        id: 1,
        side: OrderSide::Sell,
        price: 1000,
        quantity: 5,
        timestamp: Utc::now().naive_utc(),
    };

    let buy = Order {
        id: 2,
        side: OrderSide::Buy,
        price: 1100,
        quantity: 3,
        timestamp: Utc::now().naive_utc(),
    };

    ob.add_order(ask);
    let trades = ob.execute_order(buy);

    assert_eq!(trades, vec![(1, 3)]);
}
