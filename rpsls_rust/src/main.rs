mod game;

use std::str::FromStr;
use crate::game::game::game_player_role::PlayerRole;

fn main() {
    let mut buffer = String::new();
    println!("Do you want to play (y/n)?");
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            if buffer.trim().eq_ignore_ascii_case("y") {
                buffer.clear();
                println!("Which role do you want? ");
                match std::io::stdin().read_line(&mut buffer) {
                    Ok(_) => {
                        println!("You chose {}", &buffer.trim().to_uppercase());
                        let user_role= match PlayerRole::from_str(buffer.trim()) {
                            Ok(role) => role,
                            Err(_) => panic!("That's not a role name!!")
                        };
                        let (gr, res) = game::game::do_game(&user_role);
                        println!("I chose {}", gr);
                        println!("{}", res);
                    }
                    Err(error) => println!("error: {}", error)
                };
            }
        }
        Err(error) => println!("error: {}", error)
    }
}
