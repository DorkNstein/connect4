#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::collections::HashMap;

use colored::*;
mod my_io;
use my_io::game::Game;

fn start_game(player_count: usize) {
    let mut player_names: HashMap<usize, String> = HashMap::new();
    for each in 1..=player_count {
        player_names.insert(each, my_io::inputs::get_player_names(&each));
    }
    println!("{}", "Let's start!\n".bright_green());
    let mut game = Game::new();
    game.print_output();
    'outer: loop {
        for each in 1..=player_count {
            let player = each as u8;
            game.player_round(player, player_names.get(&each).unwrap());
            if game.has_found_winner() {
                break 'outer;
            }
        }
    }
}

fn main() {
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    let player_count = my_io::inputs::get_player_count();
    start_game(player_count);
    println!("{}", "GAME OVER!".bright_green());
}
