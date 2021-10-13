pub mod input_src {
    use std::str::FromStr;
    use crate::game::game::game_player_role::PlayerRole;

    pub enum InputSource {
        STDIN,
        FILE(FileBufferState)
    }

    pub struct FileBufferState {
        buffer: Vec<String>,
        ptr: usize
    }

    impl InputSource {
        pub fn from_file<>(path: &str) -> InputSource {
            let buf: Vec<String> = match std::fs::read_to_string(path) {
                Ok(buf) => buf.split_whitespace().map(|s| {String::from(s)}).collect(),
                Err(err) => panic!("{}", err)
            };
            InputSource::FILE(FileBufferState{ buffer: buf, ptr: 0 })
        }

        pub fn next_role(&mut self) -> PlayerRole {
            match self {
                InputSource::STDIN => InputSource::get_role_from_user(),
                InputSource::FILE(ref mut buffer_state) => InputSource::get_next_role_from_buffer(buffer_state)
            }
        }

        pub fn has_next_role(&self) -> bool {
            match self {
                InputSource::STDIN => InputSource::check_if_user_wants_to_play_on(),
                InputSource::FILE(guesses) => InputSource::check_if_buffer_not_used_up(guesses)
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

        fn check_if_buffer_not_used_up(buffer: &FileBufferState) -> bool {
            buffer.ptr < buffer.buffer.len()
        }

        fn get_next_role_from_buffer(buffer_state: &mut FileBufferState) -> PlayerRole {
            let mut pr: PlayerRole = PlayerRole::Spock;
            let role_name = buffer_state.buffer.get(buffer_state.ptr);
            if let Some(s) = role_name {
                match PlayerRole::from_str(s.trim()) {
                    Ok(role) => pr = role,
                    Err(_) => panic!("That is not a valid role name")
                }
            };
            buffer_state.ptr += 1;
            pr
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::game::game::game_player_role::PlayerRole;
        use crate::interaction::input_src::InputSource;

        #[test]
        fn when_guesses_read_from_file_then_okay() {
            let mut object_under_test: InputSource = InputSource::from_file("src/test_guesses.in");

            assert_eq!(true, object_under_test.has_next_role());
            assert_eq!(PlayerRole::Rock, object_under_test.next_role());
            assert_eq!(true, object_under_test.has_next_role());
            assert_eq!(PlayerRole::Paper, object_under_test.next_role());
            assert_eq!(true, object_under_test.has_next_role());
            assert_eq!(PlayerRole::Scissors, object_under_test.next_role());
            assert_eq!(true, object_under_test.has_next_role());
            assert_eq!(PlayerRole::Lizard, object_under_test.next_role());
            assert_eq!(true, object_under_test.has_next_role());
            assert_eq!(PlayerRole::Spock, object_under_test.next_role());
            assert_eq!(false, object_under_test.has_next_role());
        }
    }
}