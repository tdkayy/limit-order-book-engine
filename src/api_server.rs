use axum::{
    extract::{Extension, Json},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;


use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;

#[derive(Debug, Deserialize)]
struct NewOrder {
    price: f64,
    quantity: usize,
    side: String, // "buy" or "sell"
}

#[derive(Debug, Serialize)]
struct OrderResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Serialize)]
struct OrderBookSnapshot {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

#[derive(Debug, Serialize)]
struct OrderBookResponse {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

#[derive(Debug, Deserialize)]
struct CancelRequest {
    order_id: u64,
}

#[derive(Debug, Serialize)]
struct CancelResponse {
    success: bool,
    message: String,
}

#[debug_handler]
async fn cancel_order(
    Extension(book): Extension<SharedOrderBook>,
    Json(payload): Json<CancelRequest>,
) -> Json<CancelResponse> {
    let mut book = book.lock().await;
    let success = book.cancel_order(payload.order_id);

    if success {
        Json(CancelResponse {
            success: true,
            message: format!("Order {} cancelled successfully.", payload.order_id),
        })
    } else {
        Json(CancelResponse {
            success: false,
            message: format!("Order {} not found.", payload.order_id),
        })
    }
}

async fn get_orderbook(
    Extension(book): Extension<SharedOrderBook>,
) -> Json<OrderBookSnapshot> {
    let book = book.lock().await;

    let bids = book.bids
        .values()
        .flat_map(|orders| orders.iter().cloned())
        .collect();

    let asks = book.asks
        .values()
        .flat_map(|orders| orders.iter().cloned())
        .collect();

    Json(OrderBookSnapshot { bids, asks })
}

type SharedOrderBook = Arc<Mutex<OrderBook>>;

async fn health_check() -> &'static str {
    "API is live!"
}

#[debug_handler]
async fn place_order(
    Extension(book): Extension<SharedOrderBook>,
    Json(payload): Json<NewOrder>,
) -> Json<OrderResponse> {
    let side = match payload.side.to_lowercase().as_str() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => {
            return Json(OrderResponse {
                success: false,
                message: "Invalid order side (must be 'buy' or 'sell')".to_string(),
            });
        }
    };

    let order = Order {
        id: rand::random(), // you can replace with UUID if preferred
        side,
        price: payload.price as u64,
        quantity: payload.quantity as u32,
        timestamp: Utc::now().naive_utc(),
    };

    let mut book = book.lock().await;
    book.add_order(order); // assumes you renamed place_order -> add_order in OrderBook

    Json(OrderResponse {
        success: true,
        message: "Order placed successfully.".into(),
    })
}

async fn get_full_orderbook(
    Extension(book): Extension<SharedOrderBook>,
) -> Json<OrderBookResponse> {
    let book = book.lock().await;

    let bids = book
        .bids
        .values()
        .flat_map(|orders| orders.iter().cloned())
        .collect();

    let asks = book
        .asks
        .values()
        .flat_map(|orders| orders.iter().cloned())
        .collect();

    Json(OrderBookResponse { bids, asks })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let order_book = Arc::new(Mutex::new(OrderBook::new()));

    let app = Router::new()
    .route("/", get(health_check))
    .route("/api/orders", post(place_order))
    .route("/api/orderbook", get(get_orderbook))
    .route("/api/orders/full", get(get_full_orderbook))
    .route("/api/orders/cancel", post(cancel_order))
    .layer(CorsLayer::permissive())
    .layer(Extension(order_book));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    info!("🚀 Listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
