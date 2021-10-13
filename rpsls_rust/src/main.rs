mod game;
mod interaction;

use interaction::input_src::*;

fn main() {
    let mut inp: InputSource = InputSource::STDIN;
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
