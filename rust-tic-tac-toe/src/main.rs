extern crate piston_window;
extern crate sprite;
extern crate uuid;

use uuid::Uuid;

use std::rc::Rc;

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

use sprite::{
    Sprite,
    Scene,
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

    let black = Rc::new(Texture::from_path(
        &mut window.factory,
        "res/black.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap());

    let red = Texture::from_path(
        &mut window.factory,
        "res/red.png",
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    /* just for tests purposes, the table should be empty at the beginning */
    let mut scene = Scene::new();
    let mut black_sprite = Sprite::from_texture(black.clone());
    black_sprite.set_position(65.0, 55.0);

    let black_sprite_uuid: Uuid = scene.add_child(black_sprite);

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
            window.set_should_close(true);
        }

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {
            scene.remove_child(black_sprite_uuid);
        }
    }
}
