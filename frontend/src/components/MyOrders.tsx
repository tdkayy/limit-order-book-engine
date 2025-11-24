import React, { useEffect, useState } from 'react';
import { Order } from '@/types';

const MyOrders: React.FC = () => {
  const [myOrders, setMyOrders] = useState<Order[]>([]);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:4000/ws/orderbook');

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);

        if (data.type === 'orderbook') {
          const combined = [...data.payload.bids, ...data.payload.asks];
          setMyOrders(combined);
        }

        if (data.type === 'cancel') {
          const cancelledId = data.payload.id;
          setMyOrders((prev) => prev.filter((order) => order.id !== cancelledId));
        }
      } catch (err) {
        console.error('Invalid MyOrders message:', err);
      }
    };

    return () => {
      ws.close();
    };
  }, []);

  const cancelOrder = async (orderId: number) => {
    try {
      await fetch('/api/orders/cancel', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ order_id: orderId }),
      });
      // Removed locally as well — to keep UI responsive even if no broadcast
      setMyOrders((prev) => prev.filter((order) => order.id !== orderId));
    } catch (err) {
      console.error('Cancel failed:', err);
    }
  };

  return (
    <div className="max-w-2xl mx-auto mt-6 p-4 bg-zinc-900 rounded-lg shadow-md">
      <h2 className="text-white text-xl font-semibold mb-4 text-center">My Orders</h2>
      {myOrders.length === 0 ? (
        <p className="text-gray-300 text-center">You have no active orders.</p>
      ) : (
        <ul className="space-y-2">
          {myOrders.map((order) => (
            <li
              key={order.id}
              className="bg-zinc-800 text-white px-4 py-3 rounded flex justify-between items-center"
            >
              <div>
                <span className="font-medium">{order.side.toUpperCase()}</span>{' '}
                {order.quantity} @ {order.price}
              </div>
              <button
                onClick={() => cancelOrder(order.id)}
                className="bg-red-600 hover:bg-red-700 text-white px-3 py-1 rounded"
              >
                Cancel
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default MyOrders;
