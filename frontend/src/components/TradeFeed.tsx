import React, { useEffect, useRef, useState } from 'react';
import { Order } from '@/types';

const TradeFeed: React.FC = () => {
  const [trades, setTrades] = useState<Order[]>([]);
  const soundRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:4000/ws/orderbook');

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        if (data.type === 'trade') {
          const trade: Order = data.payload;

          setTrades((prev) => {
            // Avoid duplicates
            if (prev.some((t) => t.id === trade.id)) return prev;
            return [...prev, trade].slice(-20); // keep only the last 20
          });

          // Play sound on new trade
          if (soundRef.current) {
            soundRef.current.play().catch(() => {});
          }
        }

        if (data.type === 'cancel') {
          const cancelledId = data.payload.id;
          setTrades((prev) => prev.filter((trade) => trade.id !== cancelledId));
        }

        if (data.type === 'orderbook') {
          // Reset trade feed when server restarts
          setTrades([]);
        }
      } catch (err) {
        console.error('TradeFeed message error:', err);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="max-w-2xl mx-auto mt-6 p-4 bg-zinc-900 rounded-lg shadow-md">
      <h2 className="text-white text-xl font-semibold mb-4 text-center">Live Trade Feed</h2>
      {trades.length === 0 ? (
        <p className="text-gray-300 text-center">No recent trades.</p>
      ) : (
        <ul className="space-y-2 max-h-80 overflow-y-auto">
          {trades.map((trade) => (
            <li
              key={trade.id}
              className={`px-4 py-3 rounded flex justify-between items-center ${
                trade.side === 'buy' ? 'bg-green-700' : 'bg-red-700'
              }`}
            >
              <div>
                <span className="font-medium">{trade.side.toUpperCase()}</span>{' '}
                {trade.quantity} @ {trade.price}
              </div>
              <div className="text-sm text-zinc-200">{new Date(trade.timestamp).toLocaleTimeString()}</div>
            </li>
          ))}
        </ul>
      )}
      <audio ref={soundRef} src="/trade.mp3" preload="auto" />
    </div>
  );
};

export default TradeFeed;
