
pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

