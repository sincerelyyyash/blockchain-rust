use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio::net::TcpStream;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type PeerList = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Message>>>>;

pub async fn handle_connection(stream: WebSocketStream<TcpStream>, peers: PeerList) {
    let (mut ws_sender, mut ws_receiver) = stream.split();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    peers.lock().await.push(tx);

    let peers_clone = peers.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            let peers = peers_clone.lock().await;
            for peer in peers.iter() {
                if let Err(e) = peer.send(msg.clone()) {
                    eprintln!("Broadcast error: {}", e);
                }
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        if let Err(e) = ws_sender.send(msg).await {
            eprintln!("Send to peer failed: {}", e);
            break;
        }
    }
}