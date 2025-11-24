import { useEffect, useRef } from "react";
import { useRecoilState } from "recoil";
import { orderBookState, myOrdersState, tradeFeedState } from "@/state/atoms";
import { Order, Trade, WebSocketMessage } from "@/types";

export default function useOrderBookSocket(userId: string) {
  const [orderBook, setOrderBook] = useRecoilState(orderBookState);
  const [myOrders, setMyOrders] = useRecoilState(myOrdersState);
  const [tradeFeed, setTradeFeed] = useRecoilState(tradeFeedState);
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    ws.current = new WebSocket("ws://localhost:4000/ws");
    ws.current.onopen = () => console.log("📡 WebSocket connected");

    ws.current.onmessage = (event) => {
      const msg: WebSocketMessage = JSON.parse(event.data);

      switch (msg.type) {
        case "new_order": {
          const order: Order = msg.data;
          setOrderBook((prev): typeof orderBook => {
            const updated = { ...prev };
            updated[order.side === "buy" ? "bids" : "asks"] = [
              ...prev[order.side === "buy" ? "bids" : "asks"],
              order,
            ];
            return updated;
          });

          if (order.user_id === userId) {
            setMyOrders((prev: Order[]) => [...prev, order]);
          }
          break;
        }

        case "cancel_order": {
          const cancelledId: number = msg.data.order_id;
          const cancelledSide: "buy" | "sell" = msg.data.side;

          setOrderBook((prev): typeof orderBook => ({
            ...prev,
            [cancelledSide === "buy" ? "bids" : "asks"]: prev[
              cancelledSide === "buy" ? "bids" : "asks"
            ].filter((order: Order) => order.id !== cancelledId),
          }));

          setMyOrders((prev: Order[]) =>
            prev.filter((order: Order) => order.id !== cancelledId)
          );

          setTradeFeed((prev: Trade[]) =>
            prev.filter((trade: Trade) => trade.order_id !== cancelledId)
          );
          break;
        }

        case "new_trade": {
          const trade: Trade = msg.data;
          setTradeFeed((prev: Trade[]) => [trade, ...prev.slice(0, 49)]);
          break;
        }

        default:
          console.warn("⚠️ Unhandled message type:", msg.type);
      }
    };

    ws.current.onclose = () => console.log("🔌 WebSocket disconnected");

    return () => {
      ws.current?.close();
    };
  }, [setOrderBook, setMyOrders, setTradeFeed, userId]);
}
