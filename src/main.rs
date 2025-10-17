use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IncomingEvent {
    Strobo { event: String, state: String },
    Preset { event: String, number: String },
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("WebSocket server running on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Error during handshake");
            println!("New WebSocket connection!");

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let text = msg.into_text().unwrap();
                    //println!("Raw message: {}", text);

                    match serde_json::from_str::<IncomingEvent>(&text) {
                        Ok(event) => match event {
                            IncomingEvent::Strobo { state, .. } => {
                                if state == "down" { println!("💡 Strobo PRESSED"); }
                                else { println!("💡 Strobo RELEASED"); }
                            }
                            IncomingEvent::Preset { number, .. } => {
                                println!("🔢 Preset button pressed: {}", number);
                            }
                        },
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    }

                    // Optional: echo back
                    if let Err(e) = write.send(tungstenite::Message::Text("ok".into())).await {
                        eprintln!("Send error: {}", e);
                        break;
                    }
                }
            }

            println!("Connection closed.");
        });
    }
}
