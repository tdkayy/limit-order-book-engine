import { atom } from "recoil";
import { Order, Trade } from "@/types";

export interface OrderBookSnapshot {
  bids: Order[];
  asks: Order[];
}

export const orderBookState = atom<OrderBookSnapshot>({
  key: "orderBookState",
  default: {
    bids: [],
    asks: [],
  },
});

export const myOrdersState = atom<Order[]>({
  key: "myOrdersState",
  default: [],
});

export const tradeFeedState = atom<Trade[]>({
  key: "tradeFeedState",
  default: [],
});
