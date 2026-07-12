mod brain;
mod heartbeat;
mod state;

use brain::Brain;
use std::{thread, time::Duration};

fn main() {
    let mut embryo = Brain::new();

    println!("Cybertarr Embryo v0.0.1");

    loop {
        embryo.tick();

        thread::sleep(Duration::from_secs(1));
    }
}