use colored::*;
use std::io;

fn user_input() -> usize {
    let err_msg = "Invalid input. Please enter a number between 1 and 7.".bright_red();
    let player_input;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Should be a string value");
        let number = input.trim().parse::<usize>();
        match number {
            Ok(n) => match n {
                1..=7 => {
                    player_input = n;
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
    player_input
}

fn print_output(output: &Vec<Vec<String>>) {
    output.iter().for_each(|it| {
        let merged: String = it.iter().map(|x| x.to_owned()).collect::<String>();
        println!("{}", merged);
    });
    println!("{}", "\n");
}

fn each_round(
    output: &mut Vec<Vec<String>>,
    each_column_position: &mut Vec<usize>,
) -> (usize, usize) {
    println!(
        "{}",
        "1st player, input a number between 1 to 7".bright_cyan()
    );
    let first_input = user_input();

    let first_loc = first_input - 1;
    each_column_position[first_loc] -= 1;
    output[each_column_position[first_loc] + 1][first_loc] = format!("{}", "[x]".bright_cyan());

    print_output(output);

    println!(
        "{}",
        "2nd player, input a number between 1 to 7".bright_yellow()
    );
    let second_input = user_input();
    let second_loc = second_input - 1;
    each_column_position[second_loc] -= 1;
    output[each_column_position[second_loc] + 1][second_loc] = format!("{}", "[0]".bright_yellow());

    print_output(output);
    // println!("1st Player: {}, 2nd Player: {}", first_input, second_input);
    (first_input, second_input)
}

fn main() {
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    println!("{}", "Need 2 players to play the game".bright_purple());
    println!("{}", "Let's start!\n".bright_green());

    let mut output = vec![vec![String::from("[ ]"); 7]; 7];
    print_output(&output);
    let mut each_column_position = vec![6 as usize; 7];
    loop {
        let (_first, _second) = each_round(&mut output, &mut each_column_position);
    }
}
