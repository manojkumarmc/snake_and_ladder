use rand::prelude::*;
use std::io;
// use std::format::Display;

// use std::thread;

#[derive(Debug)]
enum ColType {
    Ladder(usize),
    Snake(usize),
}

#[derive(Debug)]
struct Col {
    col_id: usize,
    col_type: Option<ColType>,
}

#[derive(Debug)]
struct Board {
    cols: Vec<Col>,
}

impl Board {
    fn new() -> Board {
        let mut vc = Vec::new();
        for i in 0..100 {
            vc.insert(
                i,
                Col {
                    col_id: i,
                    col_type: None,
                },
            )
        }
        Board { cols: vc }
    }

    fn get_col_data(&self, col_id: usize) {
        println!("{:#?}", self.cols[col_id - 1]);
    }
}

// impl Display for Board {
//     todo!();
// }

fn main() {
    get_input();
}

fn get_input() {
    let mut buffer = String::new();
    let board = Board::new();
    loop {
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
                std::process::Command::new("clear").status().unwrap();
                let mut admin = String::new();
                io::stdin().read_line(&mut admin).unwrap();
                println!("----------------------");
                println!("[1] List players");
                println!("[2] Add player");
                match admin.trim() {
                    "1" => {
                        println!("Player list");
                    }
                    "2" => {
                        println!("Add your player");
                    },
                    _ => println!("Invalid input")
                }
            }
            _ => println!("Invalid input"),
        }
    }
}
