use std::io;

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn wait_to_proceed() {
    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char).unwrap();
}
