export interface Order {
  id: number;
  timestamp: string;
  price: number;
  quantity: number;
  side: 'buy' | 'sell';
  user_id?: string; // optional if not always present
}

export interface OrderBookSnapshot {
  bids: Order[];
  asks: Order[];
}

export interface Trade {
  id: number;
  price: number;
  quantity: number;
  side: 'buy' | 'sell';
  timestamp: string;
  order_id?: number; // used to remove trade from feed on cancel
}

export type WebSocketMessage =
  | { type: 'new_order'; data: Order }
  | { type: 'cancel_order'; data: { order_id: number; side: 'buy' | 'sell' } }
  | { type: 'new_trade'; data: Trade }
  | { type: 'orderbook'; data: OrderBookSnapshot };
