mod order;
mod order_book;
use clap::{Parser, Subcommand, ValueEnum};
use chrono::Utc;
use std::convert::TryInto;
use crate::order::{Order, OrderSide};
use crate::order_book::OrderBook;

/// A simple limit order book engine
#[derive(Parser)]
#[command(name = "LOB Engine")]
#[command(about = "A Rust-powered limit order book simulator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add an order
    Add {
        #[arg(short, long)]
        id: u64,
        #[arg(short, long)]
        side: SideArg,
        #[arg(short, long)]
        price: u64,
        #[arg(short, long)] 
        quantity: u64,
    },
    /// Cancel an order by ID
    Cancel {
        #[arg(short, long)]
        id: u64,
    },
    /// View best bid and ask
    ViewBest,
}

#[derive(ValueEnum, Clone)]
enum SideArg {
    Buy,
    Sell,
}

fn main() {
    let cli = Cli::parse();
    let mut ob = OrderBook::new();

    match cli.command {
        Commands::Add { id, side, price, quantity } => {
            let order = Order {
                id,
                side: match side {
                    SideArg::Buy => OrderSide::Buy,
                    SideArg::Sell => OrderSide::Sell,
                },
                price,
                quantity: quantity.try_into().unwrap(),
                timestamp: Utc::now().naive_utc(),
            };
            ob.add_order(order);
            println!("✅ Order added.");
        }

        Commands::Cancel { id } => {
            if ob.cancel_order(id) {
                println!("✅ Order {} cancelled.", id);
            } else {
                println!("⚠️ Order ID {} not found.", id);
            }
        }

        Commands::ViewBest => {
            println!("📈 Best Bid: {:?}", ob.get_best_bid());
            println!("📉 Best Ask: {:?}", ob.get_best_ask());
        }
    }
}

