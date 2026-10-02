pub mod display;
// pub mod cpu;

use display::Display;

fn main() {
    let width = 16;
    let height = 16;
    let my_display = Display::new(height, width);
    println!("emulador da massa");
    println!("{}", "-----".repeat(width as usize));
    my_display.print_hexa("A");
    println!("{}", "-----".repeat(width as usize));
    my_display.print_hexa("1");
    println!("{}", "-----".repeat(width as usize));
}
