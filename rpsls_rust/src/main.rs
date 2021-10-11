mod game;
mod interaction;

use interaction::input_src::*;

fn main() {
    let inp: InputSource = InputSource::STDIN;
    play(&inp);
    while inp.has_next_role() {
        play(&inp);
    }
}

fn play(input: &InputSource) {
    let user_role = input.next_role();
    let (gr, res) = game::game::do_game(&user_role);
    println!("You chose {}", user_role);
    println!("I chose {}", gr);
    println!("{}", res);
}
