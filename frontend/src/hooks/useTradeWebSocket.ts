import { useEffect, useRef, useState } from "react";

export interface TradeUpdate {
  id: number;
  side: "buy" | "sell";
  price: number;
  quantity: number;
  timestamp: string;
}

export function useTradeWebSocket() {
  const [trades, setTrades] = useState<TradeUpdate[]>([]);
  const wsRef = useRef<WebSocket | null>(null);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    const ws = new WebSocket("ws://localhost:4000/ws/trades");
    wsRef.current = ws;
    audioRef.current = new Audio("/sounds/trade.wav");

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        if (data.type === "trade") {
          setTrades((prev) => [data.payload, ...prev.slice(0, 19)]);

          // 🔊 Play trade sound
          audioRef.current?.play().catch((err) => {
            console.warn("Trade sound could not play:", err);
          });
        }

        if (data.type === "cancel" && data.payload?.id) {
          // 🧹 Remove from feed if cancelled
          setTrades((prev) => prev.filter((t) => t.id !== data.payload.id));
        }

      } catch (err) {
        console.error("Invalid WebSocket message:", err);
      }
    };

    ws.onerror = (e) => {
      console.error("WebSocket error:", e);
    };

    return () => {
      ws.close();
    };
  }, []);

  return { trades };
}
