import React from "react";
import useTradeFeed from "../hooks/useTradeFeed";

const TradeFeed = () => {
  const trades = useTradeFeed();

  return (
    <div className="bg-white rounded-lg p-4 mt-4 hover:shadow-lg transition-shadow">
      <h2 className="text-lg font-bold mb-4">Live Trade Feed</h2>
      <div className="space-y-2 max-h-80 overflow-y-auto">
        {trades.map((trade) => (
          <div
            key={trade.id}
            className={`flex justify-between items-center p-2 rounded border ${
              trade.side === "buy"
                ? "bg-green-100 border-green-400"
                : "bg-red-100 border-red-400"
            }`}
          >
            <span className="font-mono text-sm">{trade.timestamp.slice(11, 19)}</span>
            <span className="text-sm">{trade.side.toUpperCase()}</span>
            <span className="text-sm">Qty: {trade.quantity}</span>
            <span className="text-sm font-semibold">£{trade.price}</span>
          </div>
        ))}
      </div>
    </div>
  );
};

export default TradeFeed;
