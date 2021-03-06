use model::Player;
use std::io;

use crate::{
    model::{Board, ColType, MAX_COL, MAX_SNL, MIN_COL},
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
    let mut current_player_pos = true;
    let mut player_list: Vec<Player> = get_players();
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
            "3" => {
                for player in &mut player_list {
                    player.initialize();
                }
                loop {
                    println!("\n________________________");
                    let current_player = &mut player_list[current_player_pos as usize];
                    println!("{}", current_player);
                    let sv = shuffle(1, 6);
                    println!("Shuffled value is {}", sv);

                    if current_player.position + sv > MAX_COL - 1 {
                        println!("{} - out of bounds, skipping", current_player);
                    } else if current_player.position + sv == MAX_COL - 1 {
                        current_player.move_up(sv);
                        println!("🏆 {} 🏁 ", current_player);
                        wait_to_proceed();
                        break;
                    } else {
                        current_player.move_up(sv);
                        println!("{}", current_player);
                        match board.cols[current_player.position - 1].col_type {
                            Some(ColType::Snake(v)) => {
                                println!("Snake found at {}", current_player_pos);
                                println!("{}", v);
                                current_player.move_down(current_player.position - v);
                                println!("iiiiiiiiiiiiiiiiiiiii");
                                println!("{}", current_player);
                            }
                            Some(ColType::Ladder(v)) => {
                                println!("Ladder found at {}", current_player_pos);
                                println!("{}", v);
                                current_player.move_up(v - current_player.position);
                                println!("jjjjjjjjjjjjjjj");
                                println!("{}", current_player);
                            }
                            None => println!("normal col"),
                        }
                    }

                    current_player_pos = !current_player_pos;
                    wait_to_proceed();
                }
                // break;
            }
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
            "1" => loop {
                let mut admin = String::new();
                admin.clear();
                println!("----------------------------");
                println!("[4] List modified colums");
                println!("[3] Modify column property");
                println!("[2] Add player");
                println!("[1] List players");
                println!("[0] Back");
                println!("___________________________");
                io::stdin().read_line(&mut admin).unwrap();
                match admin.trim() {
                    "0" => break,
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
                    "3" => {
                        clear_screen();
                        println!("10 S 3 => Snake with head on 10 and tail on 3");
                        println!("2 L 30 => Ladder from col 2 to col 30");
                        let mut cv: Vec<String> = Vec::new();
                        loop {
                            let lv = shuffle(MIN_COL + 2, MAX_COL - 1);
                            let rv = shuffle(MIN_COL + 2, MAX_COL - 1);
                            if lv > rv {
                                let av = format!("{} S {}", lv, rv);
                                println!("{}", av);
                                if !cv.contains(&av) {
                                    cv.push(av);
                                }
                            } else {
                                let bv = format!("{} L {}", lv, rv);
                                if !cv.contains(&bv) {
                                    cv.push(bv);
                                }
                            }
                            if cv.len() == MAX_SNL {
                                break;
                            }
                        }

                        for col_mod in cv.iter() {
                            let col_mods: Vec<&str> = col_mod.split_whitespace().collect();
                            match col_modify(&col_mods, &mut board) {
                                Ok(_) => {
                                    println!("Okeyed here");
                                }
                                Err(e) => {
                                    println!("{:#?}", e);
                                }
                            }
                        }
                        println!("{:#?}", cv);
                        wait_to_proceed();
                    }
                    "4" => {
                        println!("list snake and ladder columns")
                    }
                    _ => println!("Invalid input"),
                }
            },
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

fn col_modify(cv: &Vec<&str>, brd: &mut Board) -> Result<(), &'static str> {
    if cv.len().ne(&3) {
        println!("Invalid format");
        wait_to_proceed();
        return Err("Invalid Format");
    }

    println!("{} {} {}", cv[0], cv[1], cv[2]);
    let lv = cv[0].parse::<usize>().unwrap_or(usize::MIN);
    let rv = cv[2].parse::<usize>().unwrap_or(usize::MAX);

    if lv < model::MIN_COL || lv > model::MAX_COL {
        println!("Invalid input {}", lv);
        wait_to_proceed();
        return Err("Invalid input");
    }

    if rv < model::MIN_COL || rv > model::MAX_COL {
        println!("Invalid input {}", rv);
        wait_to_proceed();
        return Err("Invalid input");
    }

    match cv[1] {
        "S" => {
            if rv.gt(&lv) {
                println!("The tail cannot be greater than the head");
                wait_to_proceed();
                Err("Invalid input tail")
            } else {
                println!("Good choice");
                brd.set_col_prop(lv, ColType::Snake(rv));
                wait_to_proceed();
                Ok(())
            }
        }
        "L" => {
            if lv.gt(&rv) {
                println!("The ladder will always have to go up");
                wait_to_proceed();
                Err("Invalid input head")
            } else {
                println!("Good choice");
                brd.set_col_prop(lv, ColType::Ladder(rv));
                wait_to_proceed();
                Ok(())
            }
        }
        _ => Err("Invalid character"),
    }
}
