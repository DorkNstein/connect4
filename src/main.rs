#![deny(clippy::all)]

use std::collections::HashMap;

use colored::Colorize;
mod my_io;
use my_io::game::Game;

use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

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
            game.player_round(player, &player_names[&each]);
            if game.has_found_winner() {
                break 'outer;
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
    println!("{}", "\nHello, Welcome to Connect 4!".bright_cyan());
    let player_count = my_io::inputs::get_player_count();
    start_game(player_count);
    println!("{}", "GAME OVER!".bright_green());
}
