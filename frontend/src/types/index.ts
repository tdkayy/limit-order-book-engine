export interface Order {
    id: number;
    timestamp: string;
    price: number;
    quantity: number;
    side: 'buy' | 'sell';
  }
  
  export interface OrderBookSnapshot {
    bids: Order[];
    asks: Order[];
  }
  