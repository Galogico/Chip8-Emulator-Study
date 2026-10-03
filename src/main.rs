pub mod display;
use std::thread;
// pub mod cpu;

use core::time;
use rand::Rng;

use display::Display;

fn main() {
    let width = 16;
    let height = 16;
    let my_display = Display::new(height, width);
    println!("emulador da massa");
    let sequence = [
        "0", "1,", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];
    loop {
        let i = rand::thread_rng().gen_range(0..sequence.len());
        thread::sleep(time::Duration::from_millis(500));
        clearscreen::clear().expect("failed to clear screen");
        // println!("{}", "-----".repeat(width as usize));
        my_display.print_hexa(sequence[i]);
    }
}
