import React, { useState } from "react";
import useMyOrders from "../hooks/useMyOrders";


const OrderForm = () => {
  const [price, setPrice] = useState("");
  const [quantity, setQuantity] = useState("");
  const [side, setSide] = useState("buy");
  const [message, setMessage] = useState("");
  const { addOrder } = useMyOrders();


  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
  
    const order = {
      price: parseFloat(price),
      quantity: parseInt(quantity),
      side: side as "buy" | "sell",
    };
  
    try {
      const res = await fetch("http://localhost:4000/api/orders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(order),
      });
  
      const data = await res.json();
      setMessage(data.message || "Unknown error");
      
      if (!res.ok) {
        console.error("❌ Order rejected by backend:", data);
      }
        
      // ✅ Immediately show in MyOrders
      if (data.success) {
        addOrder({
          ...order,
          id: data.order_id,
          side: side as "buy" | "sell",
          timestamp: new Date().toISOString(),
        });
      }
  
      setPrice("");
      setQuantity("");
    } catch (err) {
      console.error("Order submission failed", err);
      setMessage("Failed to submit order.");
    }
  };
  
  return (
    <form
      onSubmit={handleSubmit}
      className="bg-white shadow-md rounded p-6 space-y-4 hover:shadow-lg transition-shadow"
    >
      <h2 className="text-lg font-bold">Place Order</h2>

      <div className="flex space-x-2">
        <label className="flex-1">
          <span className="block text-sm font-medium text-gray-700">Price</span>
          <input
            type="number"
            step="0.01"
            value={price}
            onChange={(e) => setPrice(e.target.value)}
            className="w-full border rounded px-3 py-2"
            required
          />
        </label>

        <label className="flex-1">
          <span className="block text-sm font-medium text-gray-700">Quantity</span>
          <input
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(e.target.value)}
            className="w-full border rounded px-3 py-2"
            required
          />
        </label>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700">Side</label>
        <select
          value={side}
          onChange={(e) => setSide(e.target.value)}
          className="w-full border rounded px-3 py-2"
        >
          <option value="buy">Buy</option>
          <option value="sell">Sell</option>
        </select>
      </div>

      <button
        type="submit"
        className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded w-full transition duration-150"
      >
        Submit Order
      </button>

      {message && (
        <div className="mt-2 text-sm text-center text-gray-600">{message}</div>
      )}
    </form>
  );
};

export default OrderForm;
