import React from "react";
import { useOrderBookWebSocket } from "../hooks/useOrderBookWebSocket";

const OrderBook: React.FC = () => {
  const { bids, asks } = useOrderBookWebSocket();

  return (
    <div className="bg-white shadow-md rounded-lg p-4 hover:shadow-lg transition-shadow">
      <h2 className="text-lg font-semibold mb-3">Order Book</h2>
      <div className="grid grid-cols-2 gap-4 text-sm">
        <div>
          <h3 className="font-medium text-green-600">Bids</h3>
          <ul className="space-y-1">
            {bids.map((bid) => (
              <li key={bid.id} className="flex justify-between">
                <span>${bid.price}</span>
                <span>{bid.quantity}</span>
              </li>
            ))}
          </ul>
        </div>
        <div>
          <h3 className="font-medium text-red-600">Asks</h3>
          <ul className="space-y-1">
            {asks.map((ask) => (
              <li key={ask.id} className="flex justify-between">
                <span>${ask.price}</span>
                <span>{ask.quantity}</span>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
};

export default OrderBook;
