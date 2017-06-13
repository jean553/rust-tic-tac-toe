/// Type of pin with its pin value for calculation facilities
pub enum PinType {
    Ai(u8),
    Player(u8),
}

/// Calculate the AI pin location according to the current pin(s) on the table
/// NOTE: partially defined
pub fn find_next_pin_location(cells: &[u8]) -> Option<u8> {

    /* try to add pin at the center cell */
    if cells[4] == 0 {
        return Some(4);
    }

    /* try to add pin to one corner cell */
    for corner_address in [0, 2, 6, 8].iter() {
        if cells[*corner_address as usize] == 0 {
            return Some(*corner_address);
        }
    }

    None
}

/// Tries to finish a line
pub fn get_last_address_for_full_line(
    cells: &[u8],
    current_sum: PinType,
) -> Option<u8> {

    let sum = match current_sum {
        PinType::Ai(v) => v,
        PinType::Player(v) => v,
    };

    /* check if the game can be finished with the next pin */
    if cells[0] + cells[1] + cells[2] == sum {
        for index in 0..3 {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[3] + cells[4] + cells[5] == sum {
        for index in 3..6 {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[6] + cells[7] + cells[8] == sum {
        for index in 6..9 {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[0] + cells[3] + cells[6] == sum {
        for index in (0..7).step_by(3) {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[1] + cells[4] + cells[7] == sum {
        for index in (1..8).step_by(3) {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[2] + cells[5] + cells[8] == sum {
        for index in (2..9).step_by(3) {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[0] + cells[4] + cells[8] == sum {
        for index in (0..9).step_by(4) {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    if cells[2] + cells[4] + cells[6] == sum {
        for index in (2..7).step_by(2) {
            if cells[index] == 0 {
                return Some(index as u8);
            }
        }
    }

    None
}
