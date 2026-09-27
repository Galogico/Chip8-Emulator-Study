pub mod display;
// pub mod cpu;

use display::Display;

fn main() {
    let my_display = Display::new(8, 8);
    println!("emulador da massa");
    let sprite = Vec::from([0xFF, 0xA, 0x1A, 0x2A, 1, 2, 3, 4]);
    my_display.render(sprite);
}
