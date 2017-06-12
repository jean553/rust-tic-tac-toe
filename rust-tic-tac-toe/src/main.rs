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
    ImageSize,
};

use sprite::{
    Sprite,
    Scene,
};

fn create_pin<T: ImageSize>(
    scene: &mut Scene<T>,
    uuids: &mut Vec<Uuid>,
    texture: &Rc<T>
)
{
    let mut sprite = Sprite::from_texture(texture.clone());

    /* TODO: should be set according to the cursor location */
    sprite.set_position(100.0, 100.0);

    let uuid: Uuid = scene.add_child(sprite);
    uuids.push(uuid);
}

fn restart_game<T: ImageSize>(
    scene: &mut Scene<T>,
    uuids: &mut Vec<Uuid>,
)
{
    /* first dereference uuids to get the vector from its mutable reference;
       do not borrow the uuids vector by prepending a & because of uuids.clear();
       get a &Uuid, so we dereference it when call remove_child() */
    for uuid in &*uuids {
        scene.remove_child(*uuid);
    }

    uuids.clear();
}

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

    let mut scene: Scene<_> = Scene::new();
    let mut uuids: Vec<Uuid> = Vec::new();

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

            create_pin(
                &mut scene,
                &mut uuids,
                &black,
            );
        }

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {

            restart_game(
                &mut scene,
                &mut uuids,
            );
        }
    }
}
