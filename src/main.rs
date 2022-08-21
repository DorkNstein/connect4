use colored::*;
mod my_io;
use my_io::game::Game;

fn main() {
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    let player_count = my_io::inputs::get_player_count();

    let mut game = Game::new(7);
    println!("{}", "Let's start!\n".bright_green());
    game.print_output();
    'outer: loop {
        for each in 0..player_count {
            let player = each as u8 + 1;
            game.player_round(player);
            if game.has_found_winner() {
                game.print_game_over();
                break 'outer;
            }
        }
    }
    println!("{}", "GAME OVER!".bright_green());
}
