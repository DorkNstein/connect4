use colored::*;
use std::io;

pub const PLAY_SIZE: usize = 7;

pub fn get_player_count() -> usize {
    println!(
        "{} {}:",
        "How many players - pick 1 to".bright_green(),
        PLAY_SIZE.to_string().bright_green()
    );
    let err_msg = format!(
        "{} {}",
        "Invalid Player count. Please enter a number between 1 and".bright_red(),
        PLAY_SIZE.to_string().bright_red()
    );
    let player_count;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Should be a string value");
        let number = input.trim().parse::<usize>();
        match number {
            Ok(n) => match n {
                1..=PLAY_SIZE => {
                    player_count = n;
                    break;
                }
                _ => {
                    println!("{}", err_msg);
                    continue;
                }
            },
            Err(_) => {
                println!("{}", err_msg);
                continue;
            }
        }
    }
    player_count
}

pub fn get_player_names(player: &usize) -> String {
    println!(
        "{} {} {}:",
        "Player".bright_white(),
        player.to_string().bright_white(),
        "name".bright_white()
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Should be a string value");
    let name = input.trim().to_owned();
    name
}

pub fn user_input(each_column_position: &Vec<usize>) -> usize {
    let err_msg = format!(
        "{} {}",
        "Invalid input. Please enter a number between 1 and".bright_red(),
        PLAY_SIZE.to_string().bright_red()
    );
    let player_input;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Should be a string value");
        let number = input.trim().parse::<usize>();
        match number {
            Ok(n) => match n {
                1..=PLAY_SIZE => {
                    if each_column_position[n - 1] < PLAY_SIZE {
                        player_input = n;
                        break;
                    } else {
                        println!(
                            "{} {}",
                            "Cannot place anymore in column".bright_red(),
                            n.to_string().bright_red()
                        );
                        continue;
                    }
                }
                _ => {
                    println!("{}", err_msg);
                    continue;
                }
            },
            Err(_) => {
                println!("{}", err_msg);
                continue;
            }
        }
    }
    player_input
}
