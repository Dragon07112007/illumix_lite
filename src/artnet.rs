use std::{net::UdpSocket, sync::{Arc, Mutex}, thread};

use artnet_protocol::{ArtCommand, Output};

use lib::universe::Universe;

use crate::lib;

pub fn launch_artnet_send_thread(universe: Arc<Mutex<Universe>>) {
    thread::spawn(move || {
        let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
        println!("Created Socket for ArtNet");

        loop {
            let command = ArtCommand::Output(Output {
                data: Vec::from(universe.lock().unwrap().get_dmx_values()).into(),
                ..Output::default()
            });

            let buffer = command.write_to_buffer().expect("Failed to serialize");

            socket.send_to(&buffer, ("127.0.0.1", 6454)).unwrap();
            //println!("Sent Art-Net DMX");
            //info!("Sent Art-Net DMX");
        }
    });
}


