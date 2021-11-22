use std::io;
use rand::prelude::*;

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn wait_to_proceed() {
    let mut input_char = String::new();
    io::stdin().read_line(&mut input_char).unwrap();
}

pub fn shuffle(l_limit: usize, u_limit: usize) -> usize {
    let mut rg: usize = rand::thread_rng().gen_range(l_limit..=u_limit);
    rg
}
