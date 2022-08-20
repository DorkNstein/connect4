use std::io;

fn user_input() -> usize {
    let err_msg = "Invalid input. Please enter a number between 1 and 7.";
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
        println!("{:?}", it);
    });
}

fn each_round(output: &mut Vec<Vec<String>>, each_column_count: &mut Vec<usize>) -> (usize, usize) {
    println!("1st player, input a number between 1 to 7");
    let first_input = user_input();

    let first_loc = first_input - 1;
    each_column_count[first_loc] = each_column_count[first_loc] - 1;
    output[each_column_count[first_loc] + 1][first_loc] = String::from("[x]");

    print_output(output);

    println!("2nd player, input a number between 1 to 7");
    let second_input = user_input();
    let second_loc = second_input - 1;
    each_column_count[second_loc] = each_column_count[second_loc] - 1;
    output[each_column_count[second_loc] + 1][second_loc] = String::from("[O]");

    print_output(output);
    // println!("1st Player: {}, 2nd Player: {}", first_input, second_input);
    (first_input, second_input)
}

fn main() {
    println!("Hello, Welcome to Connect 4!");
    println!("Need two players to play the game");

    println!("Let's start!");

    let mut output = vec![vec![String::from("[ ]"); 7]; 7];
    print_output(&output);
    let mut each_column_count = vec![6 as usize; 7];
    loop {
        let (_first, _second) = each_round(&mut output, &mut each_column_count);
    }
}
