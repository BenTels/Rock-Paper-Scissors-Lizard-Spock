pub mod input_src {
    use std::str::FromStr;
    use crate::game::game::game_player_role::PlayerRole;

    pub enum InputSource {
        STDIN,
        FILE(String)
    }

    impl InputSource {
        pub fn next_role(&self) -> PlayerRole {
            match self {
                InputSource::STDIN => InputSource::get_role_from_user(),
                InputSource::FILE(path) => todo!()
            }
        }

        pub fn has_next_role(&self) -> bool {
            match self {
                InputSource::STDIN => InputSource::check_if_user_wants_to_play_on(),
                InputSource::FILE(path) => todo!()
            }
        }

        fn get_role_from_user() -> PlayerRole {
            let mut buffer = String::new();
            println!("Which role do you want (ROCK, PAPER, SCISSORS, LIZARD, SPOCK)? ");
            let result: PlayerRole = match std::io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                    let user_role = match PlayerRole::from_str(buffer.trim()) {
                        Ok(role) => role,
                        Err(_) => panic!("That's not a role name!!")
                    };
                    user_role
                },
                Err(error) => panic!("{}", error)
            };
            result
        }

        fn check_if_user_wants_to_play_on() -> bool {
            let mut buffer = String::new();
            println!("Do you want to play (Y/n)?");
            if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
                if buffer.to_ascii_uppercase().trim() == "N" {
                    return false;
                }
            }
            true
        }
    }
}