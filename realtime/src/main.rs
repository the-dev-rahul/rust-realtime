use std::str::FromStr;
use tokio_tungstenite::accept_async;

use tokio::net::TcpListener;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use serde_json;

mod serializers;
mod authentication;
mod aggregator;


static AUTHENTICATION:Lazy<Mutex<authentication::Authentication>>  = Lazy::new(||Mutex::new(authentication::Authentication::new()));
static AGGREGATOR:Lazy<Mutex<aggregator::Aggregator>>  = Lazy::new(||Mutex::new(aggregator::Aggregator::new()));

#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(  handle_connection(stream));
    }
}


async fn handle_connection(stream: tokio::net::TcpStream) {

    async fn connect(stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>>  {

        let ws_stream = accept_async(stream)
            .await?;

        println!("New WebSocket connection");

        let (_write, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            let message = message?;


            let websocket_message: serializers::WebSocketMessage = serde_json::from_str(&message.to_string()).unwrap();
            let status = serializers::Status::from_str(&websocket_message.status).unwrap();
            
            let mut auth = AUTHENTICATION.lock().unwrap();
            let mut aggrt= AGGREGATOR.lock().unwrap();
            
            match status {
                serializers::Status::Authenticate =>{
                    auth.add(websocket_message.key.clone());
                    ()},
                serializers::Status::Aggregate => {
                    if auth.verify(&websocket_message.key){
                        aggrt.add(websocket_message.data);
                    }else{
                        return Ok(());
                    }
                    ()},
                serializers::Status::Close =>{
                        auth.remove(&websocket_message.key);
                        if auth.keys.len() == 0{
                            aggrt.calculate_average();
                        }
                        return Ok(());
                    },
            }
        }
        Ok(())
    }

    match connect(stream).await{
        Ok(_result) => {
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
