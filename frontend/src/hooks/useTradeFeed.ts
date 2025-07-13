import { useEffect, useState } from "react";

type Trade = {
  id: number;
  price: number;
  quantity: number;
  side: "buy" | "sell";
  timestamp: string;
};

const useTradeFeed = () => {
  const [trades, setTrades] = useState<Trade[]>([]);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:4000/ws/trades");

    const audio = new Audio("/sounds/trade.wav");

    socket.onmessage = (event) => {
      const msg = JSON.parse(event.data);

      if (msg.type === "trade") {
        const trade: Trade = {
          id: msg.payload.id,
          price: msg.payload.price,
          quantity: msg.payload.quantity,
          side: msg.payload.side,
          timestamp: msg.payload.timestamp,
        };

        setTrades((prev) => [trade, ...prev.slice(0, 19)]);

        // ✅ Play sound
        audio.currentTime = 0;
        audio.play().catch((err) => {
          console.error("Audio play failed:", err);
        });
      }
    };

    return () => {
      socket.close();
    };
  }, []);

  return trades;
};

export default useTradeFeed;
