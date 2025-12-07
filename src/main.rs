use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Json,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
//use futures::{SinkExt, StreamExt}; for async tasks
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::CorsLayer;
use tracing::{error, info};

// Import directly from our crate modules
use limit_order_book_engine::order::{Order, OrderSide};
use limit_order_book_engine::order_book::OrderBook;

// Use a Mutex for thread safety across the web server
type SharedOrderBook = Arc<Mutex<OrderBook>>;
type TradeTx = broadcast::Sender<String>;

#[derive(Debug, Deserialize)]
struct NewOrder {
    price: f64,
    quantity: usize,
    side: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelRequest {
    pub order_id: u64,
}

#[derive(Serialize)]
struct OrderResponse {
    success: bool,
    message: String,
    order_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct CancelResponse {
    pub status: String,
    pub order_id: u64,
}

#[derive(Serialize)]
struct OrderBookResponse {
    bids: Vec<Order>,
    asks: Vec<Order>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize the engine
    let order_book = Arc::new(Mutex::new(OrderBook::new()));
    
    // Channel for broadcasting trades to frontend
    let (tx, _rx) = broadcast::channel::<String>(100);

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/orders", post(place_order))
        .route("/api/orders/cancel", post(cancel_order))
        .route("/api/orders/all", get(get_all_orders))
        .route("/api/orderbook", get(get_orderbook))
        .route("/ws/trades", get(ws_trades))
        .route("/ws/orderbook", get(orderbook_ws_handler))
        .layer(CorsLayer::permissive())
        .layer(Extension(order_book))
        .layer(Extension(tx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    info!("Server running at {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "API is live!"
}

async fn place_order(
    Extension(book): Extension<SharedOrderBook>,
    Extension(tx): Extension<TradeTx>,
    Json(payload): Json<NewOrder>,
) -> Json<OrderResponse> {
    info!("New order payload: {:?}", payload);

    let side = match payload.side.to_lowercase().as_str() {
        "buy" => OrderSide::Buy,
        "sell" => OrderSide::Sell,
        _ => {
            error!("Invalid side: {}", payload.side);
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

    // Broadcast update
    let _ = tx.send(serde_json::json!({
        "type": "order_book_update",
        "order": {
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

pub async fn cancel_order(
    Extension(book): Extension<SharedOrderBook>, // FIXED: Changed State to Extension
    Extension(tx): Extension<TradeTx>,
    Json(cancel_request): Json<CancelRequest>,
) -> impl IntoResponse {
    let order_id = cancel_request.order_id;
    tracing::info!("Cancel order request: {:?}", cancel_request);

    let mut book = book.lock().await;
    
    let removed_order = book.cancel_order(order_id);

    if removed_order.is_some() {
        // Broadcast cancellation
        let msg = serde_json::json!({
            "type": "order_cancelled",
            "order_id": order_id,
        });
        let _ = tx.send(msg.to_string());

        Json(CancelResponse {
            status: "ok".to_string(),
            order_id,
        })
    } else {
        Json(CancelResponse {
            status: "not_found".to_string(),
            order_id,
        })
    }
}

async fn get_orderbook(Extension(book): Extension<SharedOrderBook>) -> Json<OrderBookResponse> {
    let book = book.lock().await;
    // Flatten the price levels for the JSON response
    let bids = book.bids.values().flat_map(|level| level.iter().cloned()).collect();
    let asks = book.asks.values().flat_map(|level| level.iter().cloned()).collect();
    Json(OrderBookResponse { bids, asks })
}

async fn get_all_orders(Extension(book): Extension<SharedOrderBook>) -> Json<serde_json::Value> {
    let book = book.lock().await;
    let bids: Vec<_> = book.bids.values().flat_map(|level| level.iter().cloned()).collect();
    let asks: Vec<_> = book.asks.values().flat_map(|level| level.iter().cloned()).collect();
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
    let mut ticker = interval(Duration::from_millis(500)); // Faster updates (500ms)

    loop {
        ticker.tick().await;

        let snapshot = {
            let book = book.lock().await;
            serde_json::json!({
                "type": "orderbook",
                "payload": {
                    "bids": book.bids.values().flat_map(|level| level.iter().cloned()).collect::<Vec<_>>(),
                    "asks": book.asks.values().flat_map(|level| level.iter().cloned()).collect::<Vec<_>>()
                }
            })
        };

        if socket.send(Message::Text(snapshot.to_string())).await.is_err() {
            break;
        }
    }
}