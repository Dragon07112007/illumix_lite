use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use futures::{SinkExt, StreamExt};
use serde::Deserialize;

use warp::{Filter, filters::ws::Message};

use crate::{lib::fixture::FixtureComponent, patching::get_universe};

mod artnet;
#[path = "fixture_lib/lib.rs"]
mod lib;
mod patching;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum IncomingEvent {
    Strobo {
        event: String,
        state: String,
    },
    Preset {
        event: String,
        number: String,
    },
    InputValues {
        event: String,
        input1: String,
        input2: String,
    }, // <-- new variant
}

#[tokio::main]
async fn main() {
    let universe = Arc::new(Mutex::new(get_universe()));
    artnet::launch_artnet_send_thread(universe.clone());
    let universe_filter = warp::any().map(move || universe.clone());

    

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(universe_filter)
        .map(|ws: warp::ws::Ws, universe: Arc<Mutex<lib::universe::Universe>>| ws.on_upgrade(move |socket| {
            handle_websocket(socket, universe.clone())
        }));

    let static_files = warp::fs::dir("static/");

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(ws_route.or(static_files))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn handle_websocket(ws: warp::ws::WebSocket, universe: Arc<Mutex<lib::universe::Universe>>) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_str().unwrap();
                    match serde_json::from_str::<IncomingEvent>(text) {
                        Ok(event) => match event {
                            IncomingEvent::Strobo { state, .. } => {
                                if state == "down" {
                                    println!("💡 Strobo PRESSED");
                                    universe.lock().unwrap().get_fixture_by_id_mut(1).map(|fixture| {
                                        if let Some(FixtureComponent::Dimmer(dimmer)) = fixture.components.iter_mut().find(|comp| matches!(comp, FixtureComponent::Dimmer(_))) {
                                            dimmer.intensity = 255;
                                        }
                                    });
                                } else {
                                    println!("💡 Strobo RELEASED");
                                    universe.lock().unwrap().get_fixture_by_id_mut(1).map(|fixture| {
                                        if let Some(FixtureComponent::Dimmer(dimmer)) = fixture.components.iter_mut().find(|comp| matches!(comp, FixtureComponent::Dimmer(_))) {
                                            dimmer.intensity = 0;
                                        }
                                    });
                                }
                            }
                            IncomingEvent::Preset { number, .. } => {
                                println!("🔢 Preset button pressed: {}", number);
                            }
                            IncomingEvent::InputValues { input1, input2, .. } => {
                                println!(
                                    "📝 Inputs received: input1='{}', input2='{}'",
                                    input1, input2
                                );
                            }
                        },
                        Err(e) => eprintln!("Failed to parse JSON: {}", e),
                    }

                    // Optional: echo back to client
                    if let Err(e) = ws_tx.send(Message::text("ok")).await {
                        eprintln!("Send error: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("Connection closed.");
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

    let mut pan_dir = 1; // 1 = forward, -1 = backward
    let mut tilt_dir = 1; // 1 = up, -1 = down

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
        let pan_dmx = degree_to_16_bit(pan_current as u32, 540); // 540° pan range typical
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
