use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use url::Url;
use serde_json;
use tokio::io::{AsyncRead, AsyncWrite};

mod serializers;
mod singnature;


async fn send_message<S>(socket: &mut WebSocketStream<S>, status: serializers::Status,  public_key: &String,  data: f64  ) where S: AsyncRead + AsyncWrite + Unpin,{


    let websocket_message: serializers::WebSocketMessage = serializers::new(
        status.to_str(),
        public_key.clone(),
        data
    );

    let message_json = serde_json::to_string(&websocket_message).expect("Failed to serialize message");
    socket.send(Message::Text(message_json)).await.expect("Failed to send message");
}


async fn fetch_and_send_to_aggregator<S>(socket: &mut WebSocketStream<S>) 
     where S: AsyncRead + AsyncWrite + Unpin, {

    let public_key = singnature::SignatureManager::new().public_key_str();

    send_message(socket, serializers::Status::Authenticate, &public_key, 0.0).await;

    
    async fn aggreagate<S>(socket: &mut WebSocketStream<S>, public_key: &String) -> Result<(), Box<dyn std::error::Error>>   where S: AsyncRead + AsyncWrite + Unpin,{
        let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade").unwrap();
        
        let (ws_stream, _) = connect_async(url).await?;
        let (_write, mut read) = ws_stream.split();

        while let Some(message) = read.next().await  {     
            
            let message = message.unwrap().into_text().unwrap();
            let value: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&message);
            
            match value {
                Ok(data) => {
                    let s: String = data["p"].to_string().replace("\"","");
                    match s.parse::<f64>() {
                        Ok(num) => {
                            send_message(socket, serializers::Status::Aggregate, &public_key, num).await;
                            ()
                        },
                        Err(e) => println!("Failed to parse string to f64: {}", e),
                    }
                },
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}", e);
                }
            }
        }
        Ok(())
    }

    let duration = tokio::time::Duration::from_secs(10);
    match tokio::time::timeout(duration, aggreagate(socket, &public_key)).await {
        Ok(result) => {
            match result{
                Ok(_result) => {
                    println!("Operation Completed");
                },
                Err(e) => {
                    println!("Error {}", e);
                }
            }
            println!("OUT")
        },
        Err(_e) => println!("OUT"),
    }
    send_message(socket, serializers::Status::Close, &public_key, 0.0).await;
}



#[tokio::main]
async fn main() {

    let url = Url::parse("ws://localhost:9001").expect("Failed to parse URL");

    let (mut socket, _response) = connect_async(url).await.expect("Failed to connect");

    fetch_and_send_to_aggregator(&mut socket).await;
}