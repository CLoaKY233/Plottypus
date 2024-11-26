use std::env;
use rand::Rng;
use serde_json::json;
use log::{info, error};
use chrono::prelude::*;
use std::net::SocketAddr;
use futures::{StreamExt, SinkExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let addr: SocketAddr = addr.parse().expect("Invalid address");
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

pub fn get_unix_timestamp_ms() -> i64 {
    let now = Utc::now();
    now.timestamp_millis()
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    tokio::spawn(async move {
        loop {
            let random_number_1 = rand::thread_rng().gen_range(1..=1024);
            let random_number_2 = rand::thread_rng().gen_range(1..=1024);
            let timestamp = get_unix_timestamp_ms();
            let payload = json!({
                "timestamp": timestamp,
                "channel1": random_number_1,
                "channel2": random_number_2
            }).to_string();

            if let Err(e) = sender.send(Message::Text(payload)).await {
                error!("Error sending message: {}", e);
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
    });

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}