/// Calculate the AI pin location according to the current pin(s) on the table
/// NOTE: partially defined
pub fn find_next_pin_location(cells: &[u8]) -> u8 {

    /* try to add pin at the center cell */
    if cells[4] == 0 {
        return 4;
    }

    /* try to add pin to one corner cell */
    for corner_address in [0, 2, 6, 8].iter() {
        if cells[*corner_address as usize] == 0 {
            return *corner_address;
        }
    }

    return 0;
}
