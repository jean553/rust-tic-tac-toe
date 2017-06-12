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
    MouseCursorEvent,
};

use sprite::{
    Sprite,
    Scene,
};

/// Displays a new pin on the table,
/// the pin sprite is created from the given texture
fn create_pin<T: ImageSize>(
    scene: &mut Scene<T>,
    uuids: &mut Vec<Uuid>,
    texture: &Rc<T>,
    cursor_position_x: &f64,
    cursor_position_y: &f64,
)
{
    let mut sprite = Sprite::from_texture(texture.clone());

    /* cast into integers for euclidean division */
    let pin_position_x: u32 = (*cursor_position_x as u32 / 126) * 126 + 66;
    let pin_position_y: u32 = (*cursor_position_y as u32 / 114) * 114 + 57;

    sprite.set_position(
        pin_position_x as f64,
        pin_position_y as f64,
    );

    let uuid: Uuid = scene.add_child(sprite);
    uuids.push(uuid);
}

/// Removes all children from the scene and empty the uuids vector
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

            create_pin(
                &mut scene,
                &mut uuids,
                &black,
                &cursor_position_x,
                &cursor_position_y,
            );
        }

        if let Some(Button::Mouse(MouseButton::Right)) = event.release_args() {

            restart_game(
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
