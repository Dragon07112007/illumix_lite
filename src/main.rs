use std::{
    sync::{Arc, Mutex},
    time,
};


use futures::{SinkExt, StreamExt};
use serde::Deserialize;

use warp::{Filter, filters::ws::Message};

use crate::{lib::fixture::FixtureComponent, patching::get_universe};

mod artnet;
mod dmx;
mod effect;
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
    Color {
        event: String,
        color: String,
    },
    Smooth {
        event: String,
        smooth: bool,
    },
    Offset{
        event: String,
        offset: bool,
    },
    Bpm{
        event: String,
        bpm: u32,
    },
    Pan1 {
        event: String,
        pan_1: u16,
    },
    Tilt1 {
        event: String,
        tilt_1: u16,
    },
    Pan2 {
        event: String,
        pan_2: u16,
    },
    Tilt2 {
        event: String,
        tilt_2: u16,
    },
    Color1 {
        event: String,
        color_1: u8,
    },
    Color2 {
        event: String,
        color_2: u8,
    },
    Gobo1 {
        event: String,
        gobo_1: u8,
    },
    Gobo2 {
        event: String,
        gobo_2: u8,
    },
    Focus1 {
        event: String,
        focus_1: u8,
    },
    Focus2 {
        event: String,
        focus_2: u8,
    },
    Dimmer1 {
        event: String,
        dimmer_1: u8,
    },
    Dimmer2 {
        event: String,
        dimmer_2: u8,
    },

}

#[tokio::main]
async fn main() {
    let universe = Arc::new(Mutex::new(get_universe()));
    artnet::launch_artnet_send_thread(universe.clone());
    //dmx::launch_dmx_send_thread(universe.clone());
    effect::launch_present_thread(universe.clone(), time::Duration::from_millis(100));

    // Add a color swap effect for the PAR fixtures at 480 BPM (8 changes per second)
    universe
        .lock()
        .unwrap()
        .insert_present(effect::ColorSwapEffect::new(
            40.0,  // 120 BPM (2 beats per second)
            7,      // 7 fixtures
            true,   // Enable offset pattern - different starting colors
            false,  // Disable smooth transitions for testing
        ));

    let universe_filter = warp::any().map(move || universe.clone());

    let ws_route = warp::path("ws").and(warp::ws()).and(universe_filter).map(
        |ws: warp::ws::Ws, universe: Arc<Mutex<lib::universe::Universe>>| {
            ws.on_upgrade(move |socket| handle_websocket(socket, universe.clone()))
        },
    );

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
            Ok(msg) if msg.is_text() => {
                if let Err(e) = handle_text_message(&msg, &universe, &mut ws_tx).await {
                    eprintln!("Error handling message: {}", e);
                    break;
                }
            }
            Ok(_) => (), // Ignore non-text messages
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    println!("Connection closed.");
}

async fn handle_text_message(
    msg: &Message,
    universe: &Arc<Mutex<lib::universe::Universe>>,
    ws_tx: &mut futures::stream::SplitSink<warp::ws::WebSocket, Message>,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = msg.to_str().unwrap();
    let event = serde_json::from_str::<IncomingEvent>(text)?;

    match event {
        IncomingEvent::Strobo { state, .. } => handle_strobo(state, universe),
        IncomingEvent::Preset { number, .. } => {
            println!("🔢 Preset button pressed: {}", number);
        }
        IncomingEvent::InputValues { input1, input2, .. } => {
            println!("📝 Inputs received: input1='{}', input2='{}'", input1, input2);
        }
        IncomingEvent::Color { color, .. } => {
            match hex_to_rgb(&color) {
                Ok((r, g, b)) => {
                    println!("🎨 Color selected: {} -> RGB({}, {}, {})", color, r, g, b);
                    
                }
                Err(e) => {
                    eprintln!("Invalid color format '{}': {}", color, e);
                }
            }
        }
        IncomingEvent::Smooth { smooth, .. } => {
            println!("🔄 Smooth toggle: {}", smooth);
            universe.lock().unwrap().effects.iter_mut().for_each(|present| {
                if let Some(color_swap) = present.as_mut().as_any_mut().downcast_mut::<effect::ColorSwapEffect>() {
                    color_swap.smooth = smooth;
                }
            });
        }
        IncomingEvent::Offset { offset, .. } => {
            println!("🔀 Offset toggle: {}", offset);
            universe.lock().unwrap().effects.iter_mut().for_each(|present| {
                if let Some(color_swap) = present.as_mut().as_any_mut().downcast_mut::<effect::ColorSwapEffect>() {
                    color_swap.set_offset_pattern(offset);
                }
            });
        }
        IncomingEvent::Bpm { bpm, .. } => {
            println!("⏱️ BPM set to: {}", bpm);
            universe.lock().unwrap().effects.iter_mut().for_each(|present| {
                if let Some(color_swap) = present.as_mut().as_any_mut().downcast_mut::<effect::ColorSwapEffect>() {
                    color_swap.bpm = bpm as f32;
                }
            });
        }
        IncomingEvent::Pan1 { pan_1, .. } => {
            println!("↔️ Pan 1 set to: {}", pan_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Position(pos) = component {
                    pos.pan = pan_1;
                }
            });
        }
        IncomingEvent::Tilt1 { tilt_1, .. } => {
            println!("↕️ Tilt 1 set to: {}", tilt_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Position(pos) = component {
                    pos.tilt = tilt_1;
                }
            });
        }
        IncomingEvent::Pan2 { pan_2, .. } => {
            println!("↔️ Pan 2 set to: {}", pan_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Position(pos) = component {
                    pos.pan = pan_2;
                }
            });
        }
        IncomingEvent::Tilt2 { tilt_2, .. } => {
            println!("↕️ Tilt 2 set to: {}", tilt_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Position(pos) = component {
                    pos.tilt = tilt_2;
                }
            });
        }
        IncomingEvent::Color1 { color_1, .. } => {
            println!("🎨 Color 1 set to: {}", color_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::ColorWheel(color) = component {
                    color.index = color_1;
                }
            });
        }
        IncomingEvent::Color2 { color_2, .. } => {
            println!("🎨 Color 2 set to: {}", color_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::ColorWheel(color) = component {
                    color.index = color_2;
                }
            });
        }
        IncomingEvent::Gobo1 { gobo_1, .. } => {
            println!("💫 Gobo 1 set to: {}", gobo_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Gobo(gobo) = component {
                    gobo.index = gobo_1;
                }
            });
        }
        IncomingEvent::Gobo2 { gobo_2, .. } => {
            println!("💫 Gobo 2 set to: {}", gobo_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Gobo(gobo) = component {
                    gobo.index = gobo_2;
                }
            });
        }
        IncomingEvent::Focus1 { focus_1, .. } => {
            println!("🔍 Focus 1 set to: {}", focus_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Focus(focus) = component {
                    focus.value = focus_1;
                }
            });
        }
        IncomingEvent::Focus2 { focus_2, .. } => {
            println!("🔍 Focus 2 set to: {}", focus_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Focus(focus) = component {
                    focus.value = focus_2;
                }
            });
        }
        IncomingEvent::Dimmer1 { dimmer_1, .. } => {
            println!("💡 Dimmer 1 set to: {}", dimmer_1);
            universe.lock().unwrap().get_fixture_by_id_mut(8).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Dimmer(dimmer) = component {
                    dimmer.intensity = dimmer_1;
                }
            });
        }
        IncomingEvent::Dimmer2 { dimmer_2, .. } => {
            println!("💡 Dimmer 2 set to: {}", dimmer_2);
            universe.lock().unwrap().get_fixture_by_id_mut(9).unwrap().components.iter_mut().for_each(|component| {
                if let FixtureComponent::Dimmer(dimmer) = component {
                    dimmer.intensity = dimmer_2;
                }
            });
        }
    }

    // Echo back to client
    ws_tx.send(Message::text("ok")).await?;
    Ok(())
}

fn handle_strobo(state: String, universe: &Arc<Mutex<lib::universe::Universe>>) {
    let intensity = if state == "down" {
        println!("💡 Strobo PRESSED");
        255
    } else {
        println!("💡 Strobo RELEASED");
        0
    };

    universe.lock().unwrap().fixtures.iter_mut().for_each(|fixture| {
        for component in fixture.components.iter_mut() {
            if let FixtureComponent::CustomValue(cv) = component {
                if cv.name == "strobe" {
                    cv.value = intensity;
                }
            }
        }
    });
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


fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;
    Ok((r, g, b))
}   