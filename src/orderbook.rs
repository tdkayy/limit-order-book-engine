use chrono::Utc;
use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, Json, Extension},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex, RwLock};
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
type Clients = Arc<RwLock<HashSet<tokio::sync::broadcast::Sender<String>>>>;

async fn health_check() -> &'static str {
    "API is live!"
}

#[debug_handler]
async fn place_order(
    Extension(book): Extension<SharedOrderBook>,
    Extension(clients): Extension<Clients>,
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

    let snapshot = serde_json::to_string(&OrderBookSnapshot { bids, asks }).unwrap();

    let clients = clients.read().await;
    for tx in clients.iter() {
        let _ = tx.send(snapshot.clone());
    }

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

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(clients): Extension<Clients>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, clients))
}

async fn handle_socket(stream: WebSocket, clients: Clients) {
    let (mut sender, mut receiver) = stream.split();
    let (tx, mut rx) = broadcast::channel::<String>(32);

    clients.write().await.insert(tx);

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let order_book = Arc::new(Mutex::new(OrderBook::new()));
    let clients: Clients = Arc::new(RwLock::new(HashSet::new()));

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/orders", post(place_order))
        .route("/api/orderbook", get(get_orderbook))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .layer(Extension(order_book))
        .layer(Extension(clients));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    info!("🚀 Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
