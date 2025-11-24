import React, { useEffect, useState } from 'react';
import { Order } from '@/types';

const OrderBook: React.FC = () => {
  const [bids, setBids] = useState<Order[]>([]);
  const [asks, setAsks] = useState<Order[]>([]);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:4000/ws/orderbook');

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        if (data.type === 'orderbook') {
          setBids(data.payload.bids);
          setAsks(data.payload.asks);
        }

        if (data.type === 'cancel') {
          const cancelledId = data.payload.id;
          setBids((prev) => prev.filter((order) => order.id !== cancelledId));
          setAsks((prev) => prev.filter((order) => order.id !== cancelledId));
        }
      } catch (err) {
        console.error('Invalid order book message:', err);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="grid grid-cols-2 gap-4 p-4 bg-zinc-900 rounded-lg shadow-md max-w-4xl mx-auto mt-4">
      <div>
        <h2 className="text-green-400 text-lg font-semibold mb-2 text-center">Bids</h2>
        <ul className="space-y-1">
          {bids.map((order) => (
            <li
              key={order.id}
              className="bg-green-800 text-white px-3 py-2 rounded flex justify-between"
            >
              <span>{order.quantity}</span>
              <span>@</span>
              <span>{order.price}</span>
            </li>
          ))}
        </ul>
      </div>

      <div>
        <h2 className="text-red-400 text-lg font-semibold mb-2 text-center">Asks</h2>
        <ul className="space-y-1">
          {asks.map((order) => (
            <li
              key={order.id}
              className="bg-red-800 text-white px-3 py-2 rounded flex justify-between"
            >
              <span>{order.quantity}</span>
              <span>@</span>
              <span>{order.price}</span>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default OrderBook;
