import React from "react";
import Header from "./components/Header";
import OrderForm from "./components/OrderForm";
import TradeFeed from "./components/TradeFeed";
import { useOrderBookSocket } from "./hooks/useOrderBookSocket";
import OrderBook from "./components/OrderBook";
import MyOrders from "./components/MyOrders";
import "./styles/globals.css";

const App = () => {
  const orderBook = useOrderBookSocket(); // Live WebSocket hook

  return (
    <div className="min-h-screen bg-gray-100 text-gray-800">
      <Header />

      <main className="grid grid-cols-1 md:grid-cols-2 gap-6 p-6 max-w-6xl mx-auto">
        <section className="col-span-1">
          <OrderForm />
        </section>

        <section className="col-span-1">
          <OrderBook bids={orderBook.bids} asks={orderBook.asks} />
        </section>

        <section className="col-span-2">
          <TradeFeed />
        </section>
        
        <section className="col-span-2">
          <MyOrders />
        </section>
      </main>
    </div>
  );
};

export default App;
