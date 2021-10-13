mod game;
mod interaction;

use std::env;
use interaction::input_src::*;

fn main() {
    let mut inp: InputSource = get_input_source();
    play_the_game(&mut inp)
}

fn get_input_source() -> InputSource {
    let args: Vec<String> = env::args().collect();
    if args.len() == (3 as usize) && has_file_args(&args) {
        let file_path = &args[2];
        return InputSource::from_file(file_path.trim());
    }
    InputSource::STDIN
}

fn has_file_args(args: &Vec<String>) -> bool {
    (args[1] == "-f" || args[1] == "--file")
        &&
        !args[2].is_empty()
}

fn play_the_game(mut inp: &mut InputSource) {
    play(&mut inp);
    while inp.has_next_role() {
        play(&mut inp);
    }
}

fn play(input: &mut InputSource) {
    let user_role = input.next_role();
    let (gr, res) = game::game::do_game(&user_role);
    println!("You chose {}", user_role);
    println!("I chose {}", gr);
    println!("{}", res);
}
