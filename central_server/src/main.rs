use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use std::sync::Arc;
use tokio::sync::Mutex;

use central_server::handle_connection;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6000";
    let listener = TcpListener::bind(&addr).await.expect("Can't bind to port");
    println!("Central Websocket server running on ws://{}", addr);

    let peers = Arc::new(Mutex::new(vec![]));

    while let Ok((stream, _)) = listener.accept().await {
        let peers_clone = peers.clone();
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(ws_stream) => {
                    println!("New miner connected");
                    handle_connection(ws_stream, peers_clone).await;
                }
                Err(e) => eprintln!("Websocket error: {}", e)
            }
        });
    }
}