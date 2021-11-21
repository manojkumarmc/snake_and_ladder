use model::Player;
use std::io;

use crate::{
    model::{Board, ColType},
    utils::{clear_screen, shuffle, wait_to_proceed},
};

mod model;
mod utils;

// use std::thread;

fn main() {
    get_input();
}

fn get_input() {
    let mut buffer = String::new();
    let mut board = Board::new();
    let mut player_list = get_players();
    let mut current_player = true;
    loop {
        clear_screen();
        println!("\n________________________\n");
        println!("Please enter an option");
        println!("        [3] Play");
        println!("        [2] Display Columns");
        println!("        [1] Admin");
        println!("        [0] Exit");
        println!("________________________");
        /*
        thread::sleep(std::time::Duration::from_secs(1));
        */
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        println!("You  entered {}", buffer);
        match buffer.trim() {
            "3" => loop {
                println!("\n________________________");
                // for p in &player_list {
                //     println!("{:#?}", p);
                // }
                println!("player is {:?}", player_list[current_player as usize]);
                current_player = !current_player;
                println!("Shuffled value is {}", shuffle());
                wait_to_proceed();
                // break;
            },
            "2" => {
                for b in &mut board {
                    println!("{:?}", b);
                }
                wait_to_proceed();
            }
            "0" => {
                println!("Exit");
                break;
            }
            "1" => {
                let mut admin = String::new();
                admin.clear();
                println!("___________________________");
                println!("[1] List players");
                println!("[2] Add player");
                println!("[3] Modify column property");
                println!("[4] List modified colums");
                println!("___________________________");
                io::stdin().read_line(&mut admin).unwrap();
                match admin.trim() {
                    "1" => {
                        println!("Player list");
                        for pl in player_list.iter() {
                            println!("{:#?}", pl);
                        }
                        wait_to_proceed();
                    }
                    "2" => {
                        println!("Add your player");
                        if player_list.len() == 2 {
                            println!("Cant add player");
                            wait_to_proceed();
                        } else {
                            player_list.insert(
                                0,
                                Player::new(
                                    player_list.len(),
                                    format!("Player{}", player_list.len()),
                                ),
                            );
                            wait_to_proceed();
                        }
                    }
                    "3" => loop {
                        clear_screen();
                        println!("---------------------------");
                        println!("Enter cols and type");
                        println!("Format: <col> <S|L> <col>");
                        println!("10 S 3 => Snake with head on 10 and tail on 3");
                        println!("2 L 30 => Ladder from col 2 to col 30");
                        println!("___________________________");
                        let mut col_mod = String::new();
                        col_mod.clear();
                        io::stdin().read_line(&mut col_mod).unwrap();
                        let cv: Vec<&str> = col_mod.split_whitespace().collect();
                        if cv.len().ne(&3) {
                            println!("Invalid format");
                            wait_to_proceed();
                            break;
                        }
                        println!("{} {} {}", cv[0], cv[1], cv[2]);
                        let lv = cv[0].parse::<usize>().unwrap_or(usize::MIN);
                        let rv = cv[2].parse::<usize>().unwrap_or(usize::MAX);
                        if lv < model::MIN_COL || lv > model::MAX_COL {
                            println!("Invalid input {}", lv);
                            wait_to_proceed();
                            break;
                        }
                        if rv < model::MIN_COL || rv > model::MAX_COL {
                            println!("Invalid input {}", rv);
                            wait_to_proceed();
                            break;
                        }
                        match cv[1] {
                            "S" => {
                                if rv.gt(&lv) {
                                    println!("The tail cannot be greater than the head");
                                    wait_to_proceed();
                                } else {
                                    println!("Good choice");
                                    board.set_col_prop(lv, ColType::Snake(rv));
                                    wait_to_proceed();
                                    break;
                                }
                            }
                            "L" => {
                                if lv.gt(&rv) {
                                    println!("The ladder will always have to go up");
                                    wait_to_proceed();
                                } else {
                                    println!("Good choice");
                                    board.set_col_prop(lv, ColType::Ladder(rv));
                                    wait_to_proceed();
                                    break;
                                }
                            }
                            _ => println!("Invalid character"),
                        }
                    },
                    "4" => {
                        println!("list snake and ladder columns")
                    }
                    _ => println!("Invalid input"),
                }
            }
            _ => println!("Invalid input"),
        }
    }
}

fn get_players() -> Vec<Player> {
    let mut pl: Vec<Player> = Vec::new();
    for x in 1..=2 {
        pl.insert(0, Player::new(x, format!("Player{}", x)));
    }
    pl
}
