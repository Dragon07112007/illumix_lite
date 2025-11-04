use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};
use serde::Deserialize;
use tungstenite::Message;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IncomingEvent {
    Strobo { event: String, state: String },
    Preset { event: String, number: String },
    InputValues { event: String, input1: String, input2: String }, // <-- new variant
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
                    // println!("Raw message: {}", text);

                    match serde_json::from_str::<IncomingEvent>(&text) {
                        Ok(event) => match event {
                            IncomingEvent::Strobo { state, .. } => {
                                if state == "down" { println!("💡 Strobo PRESSED"); }
                                else { println!("💡 Strobo RELEASED"); }
                            }
                            IncomingEvent::Preset { number, .. } => {
                                println!("🔢 Preset button pressed: {}", number);
                            }
                            IncomingEvent::InputValues { input1, input2, .. } => {
                                println!("📝 Inputs received: input1='{}', input2='{}'", input1, input2);
                            }
                        },
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    }

                    // Optional: echo back to client
                    if let Err(e) = write.send(Message::Text("ok".into())).await {
                        eprintln!("Send error: {}", e);
                        break;
                    }
                }
            }

            println!("Connection closed.");
        });
    }
}


fn degree_to_16_bit(degree: u32, range: u32) -> u32 {
    (degree * 65535) / range
}

fn pan_tilt_sweep(
    pan_start: u32,
    pan_end: u32,
    tilt_start: u32,
    tilt_end: u32,
    pan_speed: u32,
    tilt_speed: u32,
) {
    let mut pan_current = pan_start as i32;
    let mut tilt_current = tilt_start as i32;

    let mut pan_dir = 1;   // 1 = forward, -1 = backward
    let mut tilt_dir = 1;  // 1 = up, -1 = down

    loop {
        // ---- PAN ----
        if pan_current >= pan_end as i32 {
            pan_dir = -1;
        } else if pan_current <= pan_start as i32 {
            pan_dir = 1;
        }

        // ---- TILT ----
        if tilt_current >= tilt_end as i32 {
            tilt_dir = -1;
        } else if tilt_current <= tilt_start as i32 {
            tilt_dir = 1;
        }

        // ---- Convert to 16-bit DMX values ----
        let pan_dmx = degree_to_16_bit(pan_current as u32, 540);  // 540° pan range typical
        let tilt_dmx = degree_to_16_bit(tilt_current as u32, 270); // 270° tilt range typical

        // ---- Output or send to DMX ----
        println!(
            "Pan: {:3}° → DMX {},   Tilt: {:3}° → DMX {}",
            pan_current, pan_dmx, tilt_current, tilt_dmx
        );

        // ---- Update positions ----
        pan_current += pan_dir * pan_speed as i32;
        tilt_current += tilt_dir * tilt_speed as i32;

        // ---- Wait a bit ----
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
