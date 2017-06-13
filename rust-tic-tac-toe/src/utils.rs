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
    cells: &mut [u8; 9],
)
{
    /* first dereference uuids to get the vector from its mutable reference;
       do not borrow the uuids vector by prepending a & because of uuids.clear();
       get a &Uuid, so we dereference it when call remove_child() */
    for uuid in &*uuids {
        scene.remove_child(*uuid);
    }

    uuids.clear();

    for cell in cells.iter_mut() {
        *cell = 0;
    }
}

/// Displays a new pin on the table,
/// the pin sprite is created from the given texture
pub fn create_pin<T: ImageSize>(
    scene: &mut Scene<T>,
    uuids: &mut Vec<Uuid>,
    texture: &Rc<T>,
    pin_position_x: &f64,
    pin_position_y: &f64,
)
{
    let mut sprite = Sprite::from_texture(texture.clone());

    sprite.set_position(
        *pin_position_x,
        *pin_position_y,
    );

    let uuid: Uuid = scene.add_child(sprite);
    uuids.push(uuid);
}

/// Returns the position of a pin according to the cursor position
pub fn get_pin_position_from_cursor_position(
    cursor_position_x: &f64,
    cursor_position_y: &f64,
) -> (f64, f64) {

    /* cast into integers for euclidean division */
    (
        ((*cursor_position_x as u32 / 126) * 126 + 66) as f64,
        ((*cursor_position_y as u32 / 114) * 114 + 57) as f64
    )
}

/// Returns the position of a pin according to its address on the table
pub fn get_pin_position_from_address(address: u8) -> (f64, f64){

    let horizontal_address: f64 = match address {
        0 | 3 | 6 => 0.0,
        1 | 4 | 7 => 1.0,
        _ => 2.0,
    };

    let vertical_address: f64 = match address {
        0 | 1 | 2 => 0.0,
        3 | 4 | 5 => 1.0,
        _ => 2.0,
    };

    (
        (horizontal_address * 126.0 + 66.0),
        (vertical_address * 114.0 + 57.0)
    )
}

/// Returns the pin address from its graphical position
pub fn get_pin_address_from_position(
    pin_position_x: &f64,
    pin_position_y: &f64,
) -> u8
{
    let (horizontal_address, vertical_address) = (
        (*pin_position_x as u32 / 126) as u8,
        (*pin_position_y as u32 / 114) as u8,
    );

    return vertical_address * 3 + horizontal_address;
}

/// Checks if the game is finished (one line is full of pins with same type)
pub fn is_game_finished(cells: &[u8]) -> bool {

    if cells[0] == cells[1] && cells[1] == cells[2] && cells[2] != 0 {
        return true;
    }

    if cells[3] == cells[4] && cells[4] == cells[5] && cells[5] != 0 {
        return true;
    }

    if cells[6] == cells[7] && cells[7] == cells[8] && cells[8] != 0 {
        return true;
    }

    if cells[0] == cells[3] && cells[3] == cells[6] && cells[6] != 0 {
        return true;
    }

    if cells[1] == cells[4] && cells[4] == cells[7] && cells[7] != 0 {
        return true;
    }

    if cells[2] == cells[5] && cells[5] == cells[8] && cells[8] != 0 {
        return true;
    }

    if cells[0] == cells[4] && cells[4] == cells[8] && cells[8] != 0 {
        return true;
    }

    if cells[2] == cells[4] && cells[4] == cells[6] && cells[6] != 0 {
        return true;
    }

    false
}
