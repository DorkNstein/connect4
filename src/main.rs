use colored::*;
use std::io;

const PLAY_SIZE: usize = 7;

fn get_player_letters(player_num: u8) -> String {
    let letters: [String; PLAY_SIZE] = [
        format!("{}", "[x]".bright_cyan()),
        format!("{}", "[O]".bright_yellow()),
        format!("{}", "[Z]".bright_magenta()),
        format!("{}", "[A]".green()),
        format!("{}", "[B]".blue()),
        format!("{}", "[C]".white()),
        format!("{}", "[D]".bright_blue()),
    ];
    let count = player_num as usize - 1;
    match count {
        0..=7 => letters[count].to_owned(),
        _ => letters[0].to_owned(),
    }
}

// fn get_player_count() -> usize {
//     println!(
//         "{} {}:",
//         "Enter number of players between 1 and".bright_green(),
//         PLAY_SIZE.to_string().bright_green()
//     );
//     let err_msg = format!(
//         "{} {}",
//         "Invalid Player count. Please enter a number between 1 and".bright_red(),
//         PLAY_SIZE.to_string().bright_red()
//     );
//     let player_count;
//     loop {
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Should be a string value");
//         let number = input.trim().parse::<usize>();
//         match number {
//             Ok(n) => match n {
//                 1..=PLAY_SIZE => {
//                     player_count = n;
//                     break;
//                 }
//                 _ => {
//                     println!("{}", err_msg);
//                     continue;
//                 }
//             },
//             Err(_) => {
//                 println!("{}", err_msg);
//                 continue;
//             }
//         }
//     }
//     player_count
// }

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
    player: u8,
) -> usize {
    let input_str = format!("{}", get_player_letters(player));
    let player_string: String;
    let input_msg = "st player, input a number between 1 to";
    player_string = match player {
        1 => format!(
            "{}{} {}",
            player.to_string().bright_cyan(),
            input_msg.bright_cyan(),
            PLAY_SIZE.to_string().bright_cyan()
        ),
        2 => format!(
            "{}{} {}",
            player.to_string().bright_yellow(),
            input_msg.bright_yellow(),
            PLAY_SIZE.to_string().bright_yellow()
        ),
        3 => format!(
            "{}{} {}",
            player.to_string().bright_magenta(),
            input_msg.bright_magenta(),
            PLAY_SIZE.to_string().bright_magenta()
        ),
        4 => format!(
            "{}{} {}",
            player.to_string().green(),
            input_msg.green(),
            PLAY_SIZE.to_string().green()
        ),
        5 => format!(
            "{}{} {}",
            player.to_string().blue(),
            input_msg.blue(),
            PLAY_SIZE.to_string().blue()
        ),
        6 => format!(
            "{}{} {}",
            player.to_string().white(),
            input_msg.white(),
            PLAY_SIZE.to_string().white()
        ),
        7 => format!(
            "{}{} {}",
            player.to_string().bright_blue(),
            input_msg.bright_blue(),
            PLAY_SIZE.to_string().bright_blue()
        ),
        _ => String::from(""),
    };
    println!("{}", player_string);
    let input = user_input(each_column_position);

    let position = input - 1;
    each_column_position[position] += 1;
    output[PLAY_SIZE - each_column_position[position]][position] = input_str;
    print_output(output);
    input
}

fn horizontal_check(output: &Vec<Vec<String>>, player_num: u8) -> bool {
    let mut has_won = false;
    let player_letter = get_player_letters(player_num);
    let winning_count = (PLAY_SIZE + 1) / 2;
    for row in output.iter() {
        let mut match_count = 0;
        row.iter().for_each(|each| {
            if each.to_owned() == player_letter {
                match_count += 1;
            } else if match_count < winning_count {
                match_count = 0;
            }
        });
        if match_count >= winning_count {
            has_won = true;
            break;
        }
    }
    has_won
}

fn vertical_check(output: &Vec<Vec<String>>, player_num: u8) -> bool {
    let mut has_won = false;
    let player_letter = get_player_letters(player_num);
    let winning_count = (PLAY_SIZE + 1) / 2;
    let mut each_column_count = vec![0 as usize; PLAY_SIZE];
    for row in output.iter() {
        row.iter().enumerate().for_each(|(index, each)| {
            if each.to_owned() == player_letter {
                each_column_count[index] += 1;
            } else if each_column_count[index] < winning_count {
                each_column_count[index] = 0;
            }
        });
        if each_column_count.iter().any(|x| x >= &winning_count) {
            has_won = true;
            break;
        }
    }
    has_won
}

fn diagonal_check(_output: &Vec<Vec<String>>, _player_num: u8) -> bool {
    false
}

fn has_found_winner(output: &Vec<Vec<String>>, player_num: u8) -> bool {
    horizontal_check(output, player_num)
        || (vertical_check(output, player_num) || diagonal_check(output, player_num))
}

fn main() {
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    let player_count = 2; // get_player_count();

    let mut output = vec![vec![String::from("( )"); PLAY_SIZE]; PLAY_SIZE];
    println!("{}", "Let's start!\n".bright_green());
    print_output(&output);
    let mut each_column_position = vec![0 as usize; PLAY_SIZE];
    'outer: loop {
        for each in 0..player_count {
            let player = each as u8 + 1;
            player_round(&mut output, &mut each_column_position, player);
            if has_found_winner(&output, player) {
                println!(
                    "{} {} {}",
                    "Player".bright_yellow(),
                    player.to_string().bright_yellow(),
                    "has WON!".bright_yellow()
                );
                break 'outer;
            }
        }
    }
    println!("{}", "GAME OVER!".bright_green());
}
