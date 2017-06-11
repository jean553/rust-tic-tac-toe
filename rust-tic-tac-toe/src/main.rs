extern crate piston_window;

use piston_window::{
    PistonWindow,
    WindowSettings,
    Texture,
    TextureSettings,
    Flip,
    image,
    ReleaseEvent,
    Button,
    MouseButton,
    Window,
};

fn main() {

    let mut window: PistonWindow = WindowSettings::new(
        "Rust Tic-Tac-Toe",
        [376, 340]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let table = Texture::from_path(
        &mut window.factory,
        "res/table.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {
                image(&table, context.transform, window);
            }
        );

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            window.set_should_close(true);
        }
    }
}
