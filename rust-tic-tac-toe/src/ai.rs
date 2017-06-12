/// Calculate the AI pin location according to the current pin(s) on the table
/// NOTE: partially defined
pub fn find_next_pin_location(cells: &[u8]) -> u8 {

    if cells[4] == 0 {
        return 4;
    }

    return 0;
}
