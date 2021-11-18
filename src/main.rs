use rand::prelude::*;
use std::io;

mod model;
mod utils;

// use std::thread;

fn main() {
    get_input();
}

fn get_input() {
    let mut buffer = String::new();
    let board = model::Board::new();
    loop {
        utils::clear_screen();
        println!("\n________________________\n");
        println!("Please enter an option");
        println!("        [2] Display Columns");
        println!("        [1] Admin");
        println!("        [0] Exit");
        println!("________________________");
        // thread::sleep(std::time::Duration::from_secs(1));
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        println!("You  entered {}", buffer);
        match buffer.trim() {
            "2" => {
                let mut rn = rand::thread_rng();
                println!("{:#?}", board.cols[rn.gen_range(0..99)])
            }
            "0" => {
                println!("Exit");
                break;
            }
            "1" => {
                utils::clear_screen();
                let mut admin = String::new();
                admin.clear();
                println!("----------------------");
                println!("[1] List players");
                println!("[2] Add player");
                println!("[3] Modify column property");
                io::stdin().read_line(&mut admin).unwrap();
                match admin.trim() {
                    "1" => {
                        println!("Player list");
                    }
                    "2" => {
                        println!("Add your player");
                    }
                    "3" => {
                        utils::clear_screen();
                        println!("----------------------");
                        println!("Enter cols and type");
                        println!("Format: <col> <S|L> <col>");
                        println!("10 S 3 => Snake with head on 10 and tail on 3");
                        println!("2 L 30 => Ladder from col 2 to col 30");
                        let mut col_mod = String::new();
                        col_mod.clear();
                        io::stdin().read_line(&mut col_mod).unwrap();
                        let cv: Vec<&str> = col_mod.split_whitespace().collect();
                        println!("{} {} {}", cv[0], cv[1], cv[2] );

                    }
                    _ => println!("Invalid input"),
                }
            }
            _ => println!("Invalid input"),
        }
    }
}
