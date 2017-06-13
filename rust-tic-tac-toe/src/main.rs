#![feature(step_by)]

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

    let mut play = true;

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

            if play == false {
                continue;
            }

            let (pin_position_x, pin_position_y) =
                utils::get_pin_position_from_cursor_position(
                    &cursor_position_x,
                    &cursor_position_y,
                );

            let player_pin_address =
                utils::get_pin_address_from_position(
                    &pin_position_x,
                    &pin_position_y,
                );

            cells[player_pin_address as usize] = 1;

            utils::create_pin(
                &mut scene,
                &mut uuids,
                &black,
                &pin_position_x,
                &pin_position_y,
            );

            if utils::is_game_finished(&cells) {
                play = false;
                continue;
            }

            match ai::find_next_pin_location(&cells) {
                Some(address) => {
                    let (pin_position_x, pin_position_y) =
                        utils::get_pin_position_from_address(address);

                    utils::create_pin(
                        &mut scene,
                        &mut uuids,
                        &red,
                        &pin_position_x,
                        &pin_position_y,
                    );

                    cells[address as usize] = 5;

                    if utils::is_game_finished(&cells) {
                        play = false;
                    }
                },
                None => {
                    play = false;
                }
            };
        }

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {

            utils::restart_game(
                &mut scene,
                &mut uuids,
            );

            play = true;
        }

        if let Some(position) = event.mouse_cursor_args() {
            cursor_position_x = position[0];
            cursor_position_y = position[1];
        }
    }
}
