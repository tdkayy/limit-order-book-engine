use axum::{
    extract::{Extension, Json, ws::{WebSocketUpgrade, WebSocket, Message}},
    routing::{get, post},
    response::IntoResponse,
    Router,
};
use axum_macros::debug_handler;
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::CorsLayer;
use tracing::{info, error};
use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;

type SharedOrderBook = Arc<Mutex<OrderBook>>;
type TradeTx = broadcast::Sender<String>;

#[derive(Debug, Deserialize)]
struct NewOrder {
    price: f64,
    quantity: usize,
    side: String,
}

#[derive(Debug, Serialize)]
struct OrderResponse {
    success: bool,
    message: String,
    order_id: Option<u64>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let order_book = Arc::new(Mutex::new(OrderBook::new()));
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/orders", post(place_order))
        .route("/api/orders/cancel", post(cancel_order))
        .route("/api/orders/all", get(get_all_orders))
        .route("/api/orderbook", get(get_orderbook))
        .route("/api/orders/full", get(get_full_orderbook))
        .route("/ws/trades", get(ws_trades))
        .route("/ws/orderbook", get(orderbook_ws_handler))
        .layer(CorsLayer::permissive())
        .layer(Extension(order_book))
        .layer(Extension(tx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    info!("🚀 Server running at {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "API is live!"
}

#[debug_handler]
async fn place_order(
    Extension(book): Extension<SharedOrderBook>,
    Extension(tx): Extension<TradeTx>,
    Json(payload): Json<NewOrder>,
) -> Json<OrderResponse> {
    info!("📥 New order payload: {:?}", payload);

    let side = match payload.side.to_lowercase().as_str() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => {
            error!("❌ Invalid side: {}", payload.side);
            return Json(OrderResponse {
                success: false,
                message: "Invalid order side".into(),
                order_id: None,
            });
        }
    };

    let order = Order {
        id: rand::random(),
        side,
        price: payload.price as u64,
        quantity: payload.quantity as u32,
        timestamp: Utc::now().naive_utc(),
    };

    let mut book = book.lock().await;
    book.add_order(order.clone());

    let _ = tx.send(serde_json::json!({
        "type": "trade",
        "payload": {
            "id": order.id,
            "side": payload.side.to_lowercase(),
            "price": order.price,
            "quantity": order.quantity,
            "timestamp": order.timestamp.to_string()
        }
    }).to_string());

    Json(OrderResponse {
        success: true,
        message: "Order placed successfully".into(),
        order_id: Some(order.id),
    })
}

async fn cancel_order(Json(payload): Json<CancelRequest>) -> Json<CancelResponse> {
    Json(CancelResponse {
        success: true,
        message: format!("Order {} canceled", payload.order_id),
    })
}

#[derive(Deserialize)]
struct CancelRequest {
    order_id: u64,
}

#[derive(Serialize)]
struct CancelResponse {
    success: bool,
    message: String,
}

async fn get_orderbook(Extension(book): Extension<SharedOrderBook>) -> Json<OrderBookResponse> {
    let book = book.lock().await;
    let bids = book.bids.values().flat_map(|v| v.clone()).collect();
    let asks = book.asks.values().flat_map(|v| v.clone()).collect();
    Json(OrderBookResponse { bids, asks })
}

async fn get_full_orderbook(Extension(book): Extension<SharedOrderBook>) -> Json<OrderBookResponse> {
    let book = book.lock().await;
    let bids = book.bids.values().flat_map(|v| v.clone()).collect();
    let asks = book.asks.values().flat_map(|v| v.clone()).collect();
    Json(OrderBookResponse { bids, asks })
}

#[derive(Serialize)]
struct OrderBookResponse {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

async fn get_all_orders(Extension(book): Extension<SharedOrderBook>) -> Json<serde_json::Value> {
    let book = book.lock().await;

    let bids: Vec<_> = book.bids.values().flat_map(|v| v.clone()).collect();
    let asks: Vec<_> = book.asks.values().flat_map(|v| v.clone()).collect();

    let all_orders = [bids, asks].concat();

    Json(serde_json::json!({ "orders": all_orders }))
}

async fn ws_trades(ws: WebSocketUpgrade, Extension(tx): Extension<TradeTx>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx.subscribe()))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<String>) {
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}

async fn orderbook_ws_handler(ws: WebSocketUpgrade, Extension(book): Extension<SharedOrderBook>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_orderbook_socket(socket, book))
}

async fn handle_orderbook_socket(mut socket: WebSocket, book: SharedOrderBook) {
    use tokio::time::{interval, Duration};

    let mut ticker = interval(Duration::from_millis(1000));
    loop {
        ticker.tick().await;

        let snapshot = {
            let book = book.lock().await;
            serde_json::json!({
                "type": "orderbook",
                "payload": {
                    "bids": book.bids.values().flat_map(|v| v.clone()).collect::<Vec<_>>(),
                    "asks": book.asks.values().flat_map(|v| v.clone()).collect::<Vec<_>>()
                }
            })
        };

        if socket.send(Message::Text(snapshot.to_string())).await.is_err() {
            break;
        }
    }
}
