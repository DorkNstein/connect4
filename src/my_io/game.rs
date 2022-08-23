use super::inputs;
use colored::*;

fn get_player_letters(player_num: u8) -> String {
    let letters: [String; inputs::PLAY_SIZE] = [
        format!("{}", "[X]".bright_cyan()),
        format!("{}", "[O]".bright_yellow()),
        format!("{}", "[Z]".bright_magenta()),
        format!("{}", "[A]".green()),
        format!("{}", "[B]".blue()),
        format!("{}", "[C]".white()),
        format!("{}", "[D]".bright_blue()),
        // format!("{}", "[E]".green()),
        // format!("{}", "[F]".blue()),
        // format!("{}", "[G]".white()),
        // format!("{}", "[H]".bright_blue()),
    ];
    let count = player_num as usize - 1;
    match count {
        0..=inputs::PLAY_SIZE => letters[count].to_owned(),
        _ => letters[0].to_owned(),
    }
}

fn get_player_win_highlight(player_num: u8) -> String {
    let chars = [
        "X", "O", "Z", "A", "B", "C", "D", /* "E", "F", "G", "H" */
    ];
    let letters: [String; inputs::PLAY_SIZE] = chars.map(|letter| {
        format!(
            "{}{}{}",
            "{".bright_red(),
            letter.bright_red(),
            "}".bright_red()
        )
    });
    let count = player_num as usize - 1;
    match count {
        0..=inputs::PLAY_SIZE => letters[count].to_owned(),
        _ => letters[0].to_owned(),
    }
}

pub struct Game {
    output: Vec<Vec<String>>,
    pub player: u8,
    player_name: String,
    play_size: usize,
    each_column_position: Vec<usize>,
}

impl Game {
    pub fn new() -> Self {
        let play_size = inputs::PLAY_SIZE;
        let output = vec![vec![String::from("( )"); play_size]; play_size];
        Game {
            output,
            player: 1,
            player_name: String::from(""),
            play_size,
            each_column_position: vec![0 as usize; play_size],
        }
    }

    pub fn print_output(&self) {
        self.output.iter().for_each(|it| {
            let merged: String = it.iter().map(|x| x.to_owned()).collect::<String>();
            println!("{}", merged);
        });
        println!("{}", "\n");
    }

    pub fn print_round(&self) -> String {
        let player_string: String;
        let input_msg = ", your turn";
        player_string = match self.player {
            1 => format!(
                "{}{}",
                self.player_name.bright_cyan(),
                input_msg.bright_cyan(),
            ),
            2 => format!(
                "{}{}",
                self.player_name.bright_yellow(),
                input_msg.bright_yellow(),
            ),
            3 => format!(
                "{}{}",
                self.player_name.bright_magenta(),
                input_msg.bright_magenta(),
            ),
            4 => format!("{}{}", self.player_name.green(), input_msg.green(),),
            5 => format!("{}{}", self.player_name.blue(), input_msg.blue(),),
            6 => format!("{}{}", self.player_name.white(), input_msg.white(),),
            7 => format!(
                "{}{}",
                self.player_name.bright_blue(),
                input_msg.bright_blue(),
            ),
            // 8 => format!(
            //     "{}{}",
            //     self.player_name.bright_blue(),
            //     input_msg.bright_blue(),
            // ),
            // 9 => format!(
            //     "{}{}",
            //     self.player_name.bright_blue(),
            //     input_msg.bright_blue(),
            // ),
            // 10 => format!(
            //     "{}{}",
            //     self.player_name.bright_blue(),
            //     input_msg.bright_blue(),
            // ),
            // 11 => format!(
            //     "{}{}",
            //     self.player_name.bright_blue(),
            //     input_msg.bright_blue(),
            // ),
            _ => String::from(""),
        };
        player_string
    }

    pub fn player_round(&mut self, player: u8, player_name: &String) -> usize {
        self.player = player;
        self.player_name = player_name.clone();
        let player_string = Game::print_round(&self);
        println!("{}", player_string);
        let input = inputs::user_input(&self.each_column_position);
        let position = input - 1;
        let input_str = format!("{}", get_player_letters(self.player));
        self.each_column_position[position] += 1;
        self.output[self.play_size - self.each_column_position[position]][position] = input_str;
        self.print_output();
        input
    }

    fn horizontal_check(&mut self) -> bool {
        let mut has_won = false;
        let player_letter = get_player_letters(self.player);
        let winning_count = (self.play_size + 1) / 2;
        'outer: for (y_index, row) in self.output.iter().enumerate() {
            let mut match_count = 0;
            for (_x_index, cell) in row.iter().enumerate() {
                if cell.to_owned() == player_letter {
                    match_count += 1;
                    if match_count >= winning_count {
                        for i in 0..winning_count {
                            self.output[y_index][i] = get_player_win_highlight(self.player);
                        }
                        self.print_game_over();
                        has_won = true;
                        break 'outer;
                    }
                } else if match_count < winning_count {
                    match_count = 0;
                }
            }
        }
        has_won
    }

    fn vertical_check(&mut self) -> bool {
        let mut has_won = false;
        let player_letter = get_player_letters(self.player);
        let winning_count = (self.play_size + 1) / 2;
        let mut each_column_count = vec![0 as usize; self.play_size];
        'outer: for (y_index, row) in self.output.iter().enumerate() {
            for (x_index, each) in row.iter().enumerate() {
                if each.to_owned() == player_letter {
                    each_column_count[x_index] += 1;
                    if each_column_count[x_index] == winning_count {
                        for i in 0..winning_count {
                            self.output[y_index - i][x_index] =
                                get_player_win_highlight(self.player);
                        }
                        self.print_game_over();
                        has_won = true;
                        break 'outer;
                    }
                } else if each_column_count[x_index] < winning_count {
                    each_column_count[x_index] = 0;
                }
            }
        }
        has_won
    }

    fn bottom_left_to_top_right_diagonal_check(&mut self) -> bool {
        let mut has_won = false;
        let player_letter = get_player_letters(self.player);
        let winning_count = (self.play_size + 1) / 2;
        'outer: for (y_index, row) in self.output.iter().enumerate() {
            if y_index < winning_count - 1 {
                continue 'outer;
            }
            for (x_index, cell) in row.iter().enumerate() {
                if x_index > winning_count - 1 {
                    continue;
                }
                // Check next 4 diagonal cells up from bottom left to top right starting from current cell
                if cell.to_owned() == player_letter {
                    let mut match_count = 1;
                    'inner: for each in 1..winning_count {
                        if self.output[y_index - each][x_index + each] == player_letter {
                            match_count += 1;
                            if match_count >= winning_count {
                                for i in 0..winning_count {
                                    self.output[y_index - i][x_index + i] =
                                        get_player_win_highlight(self.player);
                                }
                                self.print_game_over();
                                has_won = true;
                                break 'outer;
                            }
                        } else {
                            break 'inner;
                        }
                    }
                }
            }
        }

        has_won
    }

    fn top_left_to_bottom_right_diagonal_check(&mut self) -> bool {
        let mut has_won = false;
        let player_letter = get_player_letters(self.player);
        let winning_count = (self.play_size + 1) / 2;

        'outer: for (y_index, row) in self.output.iter().enumerate() {
            if y_index > winning_count - 1 {
                continue 'outer;
            }
            for (x_index, cell) in row.iter().enumerate() {
                if x_index > winning_count - 1 {
                    continue;
                }
                // Check next 4 diagonal cells down from top left to bottom right starting from current cell
                if cell.to_owned() == player_letter {
                    let mut match_count = 1;
                    'inner: for each in 1..winning_count {
                        if self.output[y_index + each][x_index + each] == player_letter {
                            match_count += 1;
                            if match_count >= winning_count {
                                for i in 0..winning_count {
                                    self.output[y_index + i][x_index + i] =
                                        get_player_win_highlight(self.player);
                                }
                                self.print_game_over();
                                has_won = true;
                                break 'outer;
                            }
                        } else {
                            break 'inner;
                        }
                    }
                }
            }
        }
        has_won
    }

    pub fn has_found_winner(&mut self) -> bool {
        self.horizontal_check()
            || self.vertical_check()
            || self.bottom_left_to_top_right_diagonal_check()
            || self.top_left_to_bottom_right_diagonal_check()
    }

    pub fn print_game_over(&self) {
        self.print_output();
        println!(
            "{} {}",
            self.player_name.to_string().bright_yellow(),
            "has WON!".bright_yellow()
        );
    }
}
