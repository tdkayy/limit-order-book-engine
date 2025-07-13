import { useEffect, useState } from "react";

export type MyOrder = {
  id: number;
  side: "buy" | "sell";
  price: number;
  quantity: number;
  timestamp: string;
};

const STORAGE_KEY = "my-orders";
const ID_KEY = "myOrderIds";

export default function useMyOrders() {
  const [myOrders, setMyOrders] = useState<MyOrder[]>([]);

  // Load from localStorage on init
  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) setMyOrders(JSON.parse(stored));
  }, []);

  // Save to localStorage on update
  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(myOrders));
  }, [myOrders]);

  // ✅ Recover orders from WebSocket on refresh
  useEffect(() => {
    const socket = new WebSocket("ws://localhost:4000/ws");
  
    socket.onmessage = (event) => {
      const msg = JSON.parse(event.data);
      if (msg.type === "order_book_update" && msg.order) {
        const order = msg.order;
        const storedIds: number[] = JSON.parse(localStorage.getItem(ID_KEY) || "[]");
  
        if (storedIds.includes(order.id)) {
          setMyOrders((prev) => {
            if (prev.some((o) => o.id === order.id)) return prev;
            const updated = [...prev, order];
            localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
            return updated;
          });
        }
      }
    };
  
    return () => socket.close();
  }, []);
  
  // ✅ Moved this out
  useEffect(() => {
    const storedIds = JSON.parse(localStorage.getItem(ID_KEY) || "[]");
    if (storedIds.length === 0) return;
  
    const fetchOrders = async () => {
      try {
        const res = await fetch("http://localhost:4000/api/orders/all");
        const data = await res.json();
  
        if (Array.isArray(data.orders)) {
          const matchedOrders = data.orders.filter((o: MyOrder) =>
            storedIds.includes(o.id)
          );
  
          setMyOrders(matchedOrders);
          localStorage.setItem(STORAGE_KEY, JSON.stringify(matchedOrders));
        }
      } catch (err) {
        console.error("Failed to load saved orders", err);
      }
    };
  
    fetchOrders();
  }, []);
  
  const addOrder = (order: MyOrder) => {
    setMyOrders((prev) => {
      const updated = [...prev, order];

      localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));

      // Update order IDs
      const existingIds: number[] = JSON.parse(localStorage.getItem(ID_KEY) || "[]");
      const updatedIds = [...existingIds, order.id];
      localStorage.setItem(ID_KEY, JSON.stringify(updatedIds));

      return updated;
    });
  };

  const cancelOrder = async (id: number) => {
    try {
      const res = await fetch("/api/orders/cancel", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ order_id: id }),
      });

      const result = await res.json();

      if (result.success) {
        setMyOrders((prev) => prev.filter((order) => order.id !== id));

        const updatedIds = JSON.parse(localStorage.getItem(ID_KEY) || "[]").filter((oid: number) => oid !== id);
        localStorage.setItem(ID_KEY, JSON.stringify(updatedIds));
      }
    } catch (err) {
      console.error("Cancel failed", err);
    }
  };

  const clearAll = () => {
    setMyOrders([]);
    localStorage.removeItem(STORAGE_KEY);
    localStorage.removeItem(ID_KEY);
  };

  return { myOrders, addOrder, cancelOrder, clearAll };
}
