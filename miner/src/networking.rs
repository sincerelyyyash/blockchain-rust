use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::Message;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use serde_json;

pub async fn start_networking(blockchain: &mut Blockchain) {
    let url = "ws://127.0.0.1:6000";
    println!("Connecting to central server at {}", url);

    let (mut socket, _) = connect_async(url).await.expect("Websocket failed");

    loop {
        if let Some(Ok(msg)) = socket.next().await {
            if let Message::Text(txt) = msg {
                if let Ok(txn) = serde_json::from_str::<Transaction>(&txt) {
                    println!("Received txn from network: {:?}", txn);
                    blockchain.add_transaction(txn);
                    
                    let block = blockchain.mine_block();
                    println!("Mined new block: {:?}", block.hash);

                    let block_msg = serde_json::to_string(&block).unwrap();
                    socket.send(Message::Text(block_msg)).await.unwrap();
                }
            }
        }
    }
}