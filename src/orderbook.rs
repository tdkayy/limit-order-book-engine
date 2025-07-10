use chrono::Utc;
use axum::{
    extract::{Json, Extension},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;

use limit_order_book_engine::order_book::{Order, OrderBook, OrderSide};

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
                message: "Invalid side".into(),
            });
        }
    };

    let order = Order {
        id: rand::random(),
        timestamp: Utc::now().naive_utc(),
        price: payload.price as u64,
        quantity: payload.quantity as u32,
        side,
    };

    let mut book = book.lock().await;
    book.add_order(order);

    Json(OrderResponse {
        success: true,
        message: "Order placed successfully.".into(),
    })
}

async fn get_orderbook(Extension(book): Extension<SharedOrderBook>) -> Json<OrderBookSnapshot> {
    let book = book.lock().await;

    let bids = book
        .bids
        .iter()
        .flat_map(|(_, level)| level.iter().cloned())
        .collect::<Vec<_>>();
    let asks = book
        .asks
        .iter()
        .flat_map(|(_, level)| level.iter().cloned())
        .collect::<Vec<_>>();

    Json(OrderBookSnapshot { bids, asks })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let order_book = Arc::new(Mutex::new(OrderBook::new()));

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/orders", post(place_order))
        .route("/api/orderbook", get(get_orderbook))
        .layer(CorsLayer::permissive())
        .layer(Extension(order_book));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    info!("🚀 Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
