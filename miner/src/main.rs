mod blockchain;
mod transaction;
mod wallet;
mod networking;
mod utils;

use blockchain::Blockchain;
use networking::start_networking;

#[tokio::main]
async fn main(){
    println!("Miner starting. . . .");

    let mut blockchain = Blockchain::new();
    start_networking(&mut blockchain).await;
}