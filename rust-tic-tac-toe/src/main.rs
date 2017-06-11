extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Tic-Tac-Toe",
        [400, 400]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(value) = window.next() {
    }
}
