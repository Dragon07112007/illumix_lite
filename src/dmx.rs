use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn launch_dmx_send_thread(universe: Arc<Mutex<crate::lib::universe::Universe>>) {
    std::thread::spawn(move || {
        println!("Connecting to Serial Device...");

        let mut port = serialport::new("/dev/ttyUSB0", 250000)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::Two)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Couldnt Open Serial Port");

        println!("Connected to Serial Device");

        println!("Sending DMX Data...");
        loop {
            // Send DMX Break
            port.set_break().ok();
            thread::sleep(Duration::from_micros(120));
            port.clear_break().ok();
            thread::sleep(Duration::from_micros(12));
            // Copy data from shared Dmx Universe
            let mut new_channels: [u8; 513] = [0; 513];
            {
                new_channels[1..].copy_from_slice(&universe.lock().unwrap().get_dmx_values());
            }
            // write to port
            port.write_all(&new_channels)
                .expect("Failed to write DMX data");
            port.flush().ok();
            thread::sleep(Duration::from_millis(25)); // ~40 FPS
        }
    });
}
