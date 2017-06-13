extern crate piston_window;
extern crate sprite;
extern crate uuid;

use std::rc::Rc;

use uuid::Uuid;

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
    MouseCursorEvent,
};

use sprite::{
    Scene,
};

mod utils;
mod ai;

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

    let black = Rc::new(Texture::from_path(
        &mut window.factory,
        "res/black.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap());

    let red = Rc::new(Texture::from_path(
        &mut window.factory,
        "res/red.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap());

    let mut scene: Scene<_> = Scene::new();
    let mut uuids: Vec<Uuid> = Vec::new();
    let mut cells = [0; 9];

    let mut cursor_position_x: f64 = 0.0;
    let mut cursor_position_y: f64 = 0.0;

    while let Some(event) = window.next() {

        window.draw_2d(
            &event,
            |context, window| {
                image(
                    &table,
                    context.transform,
                    window
                );
            }
        );

        window.draw_2d(
            &event,
            |context, window| {
                scene.draw(
                    context.transform,
                    window
                );
            }
        );

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {

            let (pin_position_x, pin_position_y) =
                utils::get_pin_position_from_cursor_position(
                    &cursor_position_x,
                    &cursor_position_y,
                );

            utils::create_pin(
                &mut scene,
                &mut uuids,
                &black,
                &pin_position_x,
                &pin_position_y,
            );

            let ai_pin_address = ai::find_next_pin_location(&cells);

            let (pin_position_x, pin_position_y) =
                utils::get_pin_position_from_address(ai_pin_address);

            utils::create_pin(
                &mut scene,
                &mut uuids,
                &red,
                &pin_position_x,
                &pin_position_y,
            );

            cells[ai_pin_address as usize] = 2;
        }

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {

            utils::restart_game(
                &mut scene,
                &mut uuids,
            );
        }

        if let Some(position) = event.mouse_cursor_args() {
            cursor_position_x = position[0];
            cursor_position_y = position[1];
        }
    }
}
