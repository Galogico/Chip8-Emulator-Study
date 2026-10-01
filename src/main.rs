pub mod display;
// pub mod cpu;

use display::Display;

fn main() {
    let my_display = Display::new(8, 8);
    println!("emulador da massa");
    my_display.print_hexa("A");
    println!("- - - - - - - - - - - - - - - - ");
    my_display.print_hexa("1");
}
