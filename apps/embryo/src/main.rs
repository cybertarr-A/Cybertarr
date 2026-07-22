mod brain;
mod heartbeat;
mod world;
mod physiology;
mod homeostasis;
mod logger;

use brain::Brain;
use std::{thread, time::Duration};

fn main() {
    println!("=================================");
    println!("Cybertarr");
    println!("Digital Embryo");
    println!("Version 0.0.1-alpha");
    println!("=================================\n");

    let mut embryo = Brain::new();

    loop {
        embryo.tick();

        thread::sleep(Duration::from_secs(1));
    }
}