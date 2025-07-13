import React, { useEffect, useState } from "react";

type Trade = {
  id: number;
  price: number;
  quantity: number;
  side: "buy" | "sell";
  timestamp: string;
};

const LiveFeed = () => {
  const [trades, setTrades] = useState<Trade[]>([]);

  useEffect(() => {
    const ws = new WebSocket("ws://localhost:4000/ws/trades");

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);

      if (data.type === "trade") {
        const newTrade: Trade = {
          id: data.payload.id,
          price: data.payload.price,
          quantity: data.payload.quantity,
          side: data.payload.side,
          timestamp: data.payload.timestamp,
        };

        setTrades((prev) => [newTrade, ...prev.slice(0, 49)]); // Max 50 trades
      }
    };

    return () => ws.close();
  }, []);

  return (
    <div className="bg-white shadow-md rounded p-4">
      <h2 className="text-lg font-bold mb-2">Live Feed</h2>
      <ul className="max-h-64 overflow-y-scroll text-sm space-y-1">
        {trades.map((trade) => (
          <li
            key={trade.id}
            className={`flex justify-between px-2 py-1 rounded ${
              trade.side === "buy" ? "bg-green-100" : "bg-red-100"
            }`}
          >
            <span>{trade.side.toUpperCase()}</span>
            <span>{trade.quantity} @ {trade.price}</span>
            <span className="text-gray-500">{new Date(trade.timestamp).toLocaleTimeString()}</span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default LiveFeed;
