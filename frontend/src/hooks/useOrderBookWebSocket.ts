import { useEffect, useRef, useState } from "react";
import type { Order } from "../types";

export function useOrderBookWebSocket() {
  const [bids, setBids] = useState<Order[]>([]);
  const [asks, setAsks] = useState<Order[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket("ws://localhost:4000/ws/orderbook");
    wsRef.current = ws;

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.type === "orderbook") {
          setBids(data.payload.bids);
          setAsks(data.payload.asks);
        }
      } catch (err) {
        console.error("Invalid WebSocket message:", err);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return { bids, asks };
}
