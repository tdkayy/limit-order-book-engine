import { useEffect, useState } from "react";

export interface Order {
  id: number;
  price: number;
  quantity: number;
  side: "Buy" | "Sell";
  timestamp: string;
}

export interface OrderBookSnapshot {
  bids: Order[];
  asks: Order[];
}

export const useOrderBookSocket = (url: string = "ws://localhost:4000/ws") => {
  const [orderBook, setOrderBook] = useState<OrderBookSnapshot>({ bids: [], asks: [] });

  useEffect(() => {
    const socket = new WebSocket(url);

    socket.onmessage = (event) => {
      const data = JSON.parse(event.data) as OrderBookSnapshot;
      setOrderBook(data);
    };

    socket.onerror = (err) => {
      console.error("WebSocket error:", err);
    };

    return () => {
      socket.close();
    };
  }, [url]);

  return orderBook;
};
