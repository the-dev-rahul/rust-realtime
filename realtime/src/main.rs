use std::str::FromStr;
use authentication::Authentication;
use tokio_tungstenite::accept_async;

use tokio::net::TcpListener;
use futures_util::StreamExt;
use std::sync::Mutex;
use std::sync::Arc;
use serde_json;

mod serializers;
mod authentication;
mod aggregator;


#[tokio::main]
async fn main() {

    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let auth  = Arc::new(Mutex::new(authentication::Authentication::new()));
    let aggrt = Arc::new(Mutex::new(aggregator::Aggregator::new()));


    while let Ok((stream, _)) = listener.accept().await {
        let  auth_clone =  Arc::clone(&auth); 
        let  aggrt: Arc<Mutex<aggregator::Aggregator>> =  Arc::clone(&aggrt); 
        tokio::spawn(  handle_connection(stream, auth_clone, aggrt));
    }
}


async fn handle_connection(stream: tokio::net::TcpStream, auth: Arc<Mutex<Authentication>>, aggrt : Arc<Mutex<aggregator::Aggregator>>) {

    async fn connect(stream: tokio::net::TcpStream, auth: Arc<Mutex<Authentication>>, aggrt : Arc<Mutex<aggregator::Aggregator>>) -> Result<(), Box<dyn std::error::Error>>  {

        let ws_stream = accept_async(stream)
            .await?;

        println!("New WebSocket connection");

        let (_write, mut read) = ws_stream.split();

        while let Some(message) = read.next().await {
            let message = message?;


            let websocket_message: serializers::WebSocketMessage = serde_json::from_str(&message.to_string()).unwrap();
            let status = serializers::Status::from_str(&websocket_message.status).unwrap();
            
            let mut auth = auth.lock().unwrap();
            let mut aggrt= aggrt.lock().unwrap();
            
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

    match connect(stream, auth, aggrt).await{
        Ok(_result) => {
        },
        Err(e) => {
            println!("Error {}", e);
        }
    }
}
