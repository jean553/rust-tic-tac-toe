use std::rc::Rc;

use uuid::Uuid;

use piston_window::{
    ImageSize,
};

use sprite::{
    Sprite,
    Scene,
};

/// Removes all children from the scene and empty the uuids vector
pub fn restart_game<T: ImageSize>(
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

/// Displays a new pin on the table,
/// the pin sprite is created from the given texture
pub fn create_pin<T: ImageSize>(
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

