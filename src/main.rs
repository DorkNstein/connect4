use colored::*;
use std::io;

const PLAY_SIZE: usize = 7;

fn user_input(each_column_position: &Vec<usize>) -> usize {
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

fn print_output(output: &Vec<Vec<String>>) {
    output.iter().for_each(|it| {
        let merged: String = it.iter().map(|x| x.to_owned()).collect::<String>();
        println!("{}", merged);
    });
    println!("{}", "\n");
}

fn player_round(
    output: &mut Vec<Vec<String>>,
    each_column_position: &mut Vec<usize>,
    player: u8, // 1 or 2
) -> usize {
    let player_string = match player {
        1 => format!(
            "{}{} {}",
            player.to_string().bright_cyan(),
            "st player, input a number between 1 to".bright_cyan(),
            PLAY_SIZE.to_string().bright_cyan()
        ),
        2 => format!(
            "{}{} {}",
            player.to_string().bright_yellow(),
            "st player, input a number between 1 to".bright_yellow(),
            PLAY_SIZE.to_string().bright_yellow()
        ),
        _ => format!("{}", "Wrong player number".red()),
    };
    println!("{}", player_string);
    let input = user_input(each_column_position);

    let position = input - 1;
    each_column_position[position] += 1;

    let input_str = match player {
        1 => format!("{}", "[x]".bright_cyan()),
        2 => format!("{}", "[0]".bright_yellow()),
        _ => String::from(""),
    };
    output[PLAY_SIZE - each_column_position[position]][position] = input_str;
    print_output(output);
    input
}

fn main() {
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    println!("{}", "Need 2 players to play the game".bright_purple());
    println!("{}", "Let's start!\n".bright_green());

    let mut output = vec![vec![String::from("( )"); PLAY_SIZE]; PLAY_SIZE];
    print_output(&output);
    let mut each_column_position = vec![0 as usize; PLAY_SIZE];
    loop {
        player_round(&mut output, &mut each_column_position, 1);
        player_round(&mut output, &mut each_column_position, 2);
    }
}
