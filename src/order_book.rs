use crate::order::{Order, OrderSide};
use std::collections::{BTreeMap, HashMap, VecDeque};

pub type PriceLevel = VecDeque<Order>;

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<u64, PriceLevel>,
    pub asks: BTreeMap<u64, PriceLevel>,
    pub order_locations: HashMap<u64, u64>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_locations: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price = order.price;
        let id = order.id;

        self.order_locations.insert(id, price);

        let book_side = match order.side {
            OrderSide::Buy => &mut self.bids,
            OrderSide::Sell => &mut self.asks,
        };

        book_side
            .entry(price)
            .or_insert_with(VecDeque::new)
            .push_back(order);
    }

    pub fn get_best_bid(&self) -> Option<u64> {
        self.bids.keys().next_back().copied()
    }

    pub fn get_best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }

    pub fn cancel_order(&mut self, order_id: u64) -> Option<Order> {
        let price = self.order_locations.remove(&order_id)?;

        if let Some(order) = Self::remove_from_side(&mut self.bids, price, order_id) {
            return Some(order);
        }

        if let Some(order) = Self::remove_from_side(&mut self.asks, price, order_id) {
            return Some(order);
        }

        None
    }

    fn remove_from_side(
        book: &mut BTreeMap<u64, PriceLevel>,
        price: u64,
        order_id: u64,
    ) -> Option<Order> {
        if let Some(level) = book.get_mut(&price) {
            if let Some(idx) = level.iter().position(|o| o.id == order_id) {
                let removed = level.remove(idx);
                if level.is_empty() {
                    book.remove(&price);
                }
                return removed;
            }
        }
        None
    }

    pub fn execute_order(&mut self, mut incoming: Order) -> Vec<(u64, u32)> {
        let mut fills = Vec::new();

        {
            let book = match incoming.side {
                OrderSide::Buy => &mut self.asks,
                OrderSide::Sell => &mut self.bids,
            };

            let mut empty_levels = Vec::new();

            for (&price, level) in book.iter_mut() {
                let match_possible = match incoming.side {
                    OrderSide::Buy => price <= incoming.price,
                    OrderSide::Sell => price >= incoming.price,
                };

                if !match_possible {
                    break;
                }

                while let Some(resting) = level.front_mut() {
                    let traded_qty = incoming.quantity.min(resting.quantity);

                    incoming.quantity -= traded_qty;
                    resting.quantity -= traded_qty;
                    fills.push((resting.id, traded_qty));

                    if resting.quantity == 0 {
                        self.order_locations.remove(&resting.id);
                        level.pop_front();
                    }

                    if incoming.quantity == 0 {
                        break;
                    }
                }

                if level.is_empty() {
                    empty_levels.push(price);
                }

                if incoming.quantity == 0 {
                    break;
                }
            }

            for price in empty_levels {
                book.remove(&price);
            }
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
    use chrono::Utc;

    fn new_order(side: OrderSide, price: u64, quantity: u32) -> Order {
        Order {
            id: rand::random(),
            side,
            price,
            quantity,
            timestamp: Utc::now().naive_utc(),
        }
    }

    #[test]
    fn test_simple_match() {
        let mut book = OrderBook::new();
        
        let sell_order = new_order(OrderSide::Sell, 100, 10);
        book.add_order(sell_order);

        let buy_order = new_order(OrderSide::Buy, 100, 10);
        let fills = book.execute_order(buy_order);

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].1, 10);
        assert!(book.asks.is_empty());
        assert!(book.bids.is_empty());
    }

    #[test]
    fn test_partial_fill() {
        let mut book = OrderBook::new();
        
        book.add_order(new_order(OrderSide::Sell, 100, 20));

        let buy_order = new_order(OrderSide::Buy, 100, 10);
        let fills = book.execute_order(buy_order);

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].1, 10);

        let best_ask = book.asks.values().next().unwrap();
        assert_eq!(best_ask[0].quantity, 10);
    }

    #[test]
    fn test_price_priority() {
        let mut book = OrderBook::new();
        
        let sell_cheap = new_order(OrderSide::Sell, 100, 10);
        let id_cheap = sell_cheap.id;
        book.add_order(sell_cheap);

        let sell_expensive = new_order(OrderSide::Sell, 101, 10);
        book.add_order(sell_expensive);

        let buy_order = new_order(OrderSide::Buy, 101, 10);
        let fills = book.execute_order(buy_order);

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].0, id_cheap);
    }
}