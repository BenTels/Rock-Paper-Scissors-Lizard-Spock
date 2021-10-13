pub mod game {
    use rand::Rng;
    use crate::game::game::game_player_role::PlayerRole;
    use crate::game::game::game_result::GameResult;

    pub fn do_game(user_role: &PlayerRole) -> (PlayerRole, GameResult) {
        let mut rng = rand::thread_rng();
        let game_role: PlayerRole = rng.gen();
        let result = user_role.compare(&game_role);
        (game_role, result)
    }

    pub mod game_result {
        use std::fmt::{Display, Formatter};

        #[derive(Debug)]
        #[derive(PartialEq)]
        pub enum GameResult {
            Win, Lose, Draw,
        }

        impl Display for GameResult {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}",
                    match self {
                        GameResult::Win => "You WIN!!",
                        GameResult::Lose => "You lose...",
                        GameResult::Draw => "It's a DRAW."
                    }
                )
            }
        }

        #[cfg(test)]
        mod tests {
            use crate::game::game::game_result::GameResult;

            #[test]
            fn when_win_then_display_correctly() {
                assert_eq!("You WIN!!", format!("{}", GameResult::Win));
            }

            #[test]
            fn when_lose_then_display_correctly() {
                assert_eq!("You lose...", format!("{}", GameResult::Lose));
            }

            #[test]
            fn when_draw_then_display_correctly() {
                assert_eq!("It's a DRAW.", format!("{}", GameResult::Draw));
            }
        }
    }

    pub mod game_player_role {
        use rand::Rng;
        use rand::distributions::{Distribution, Standard};
        use std::fmt::{Display, Formatter};
        use std::str::FromStr;

        use super::game_result::GameResult;

        #[derive(Debug)]
        #[derive(PartialEq)]
        #[derive(Eq)]
        #[derive(Hash)]
        pub enum PlayerRole {
            Rock, Paper, Scissors, Lizard, Spock
        }

        impl PlayerRole {
           fn index(&self) -> u8 {
               match self {
                   PlayerRole::Scissors => 0,
                   PlayerRole::Paper => 1,
                   PlayerRole::Rock => 2,
                   PlayerRole::Lizard => 3,
                   PlayerRole::Spock => 4
               }
           }

            pub fn compare(&self, other: &PlayerRole) -> GameResult {
                let diff = if self.index() <= other.index() { other.index() - self.index() } else { self.index() -  other.index() + 1};
                return if diff == 0 {
                    GameResult::Draw
                } else if diff % 2 == 0 {
                    GameResult::Lose
                } else {
                    GameResult::Win
                }
            }
        }

        impl Display for PlayerRole {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}",
                    match self {
                        PlayerRole::Rock => "ROCK",
                        PlayerRole::Paper => "PAPER",
                        PlayerRole::Scissors => "SCISSORS",
                        PlayerRole::Lizard => "LIZARD",
                        PlayerRole::Spock => "SPOCK",
                    }
                )
            }
        }

        impl FromStr for PlayerRole {
            type Err = ();

            fn from_str(name: &str) -> Result<Self, Self::Err> {
                match name.to_ascii_uppercase().as_str() {
                    "ROCK" => Ok(PlayerRole::Rock),
                    "PAPER" => Ok(PlayerRole::Paper),
                    "SCISSORS" => Ok(PlayerRole::Scissors),
                    "LIZARD" => Ok(PlayerRole::Lizard),
                    "SPOCK" => Ok(PlayerRole::Spock),
                   _ => Err(())
                }
            }
        }

        impl Distribution<PlayerRole> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PlayerRole {
                let draw = rng.gen_range(0..5);
                match draw {
                    0 => PlayerRole::Rock,
                    1 => PlayerRole::Paper,
                    2 => PlayerRole::Scissors,
                    3 => PlayerRole::Lizard,
                    4 => PlayerRole::Spock,
                    _ => panic!("Drawing randomly from range does not work!!")
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use std::collections::HashMap;
            use super::*;

            #[test]
            fn when_roles_equal_then_draw() {
                assert_eq!(GameResult::Draw, PlayerRole::Rock.compare(&PlayerRole::Rock));
                assert_eq!(GameResult::Draw, PlayerRole::Paper.compare(&PlayerRole::Paper));
                assert_eq!(GameResult::Draw, PlayerRole::Scissors.compare(&PlayerRole::Scissors));
                assert_eq!(GameResult::Draw, PlayerRole::Lizard.compare(&PlayerRole::Lizard));
                assert_eq!(GameResult::Draw, PlayerRole::Spock.compare(&PlayerRole::Spock));
            }

            #[test]
            fn when_winning_role_does_comparison_then_win() {
                assert_eq!(GameResult::Win, PlayerRole::Rock.compare(&PlayerRole::Scissors));
                assert_eq!(GameResult::Win, PlayerRole::Rock.compare(&PlayerRole::Lizard));
                assert_eq!(GameResult::Win, PlayerRole::Paper.compare(&PlayerRole::Rock));
                assert_eq!(GameResult::Win, PlayerRole::Paper.compare(&PlayerRole::Spock));
                assert_eq!(GameResult::Win, PlayerRole::Scissors.compare(&PlayerRole::Paper));
                assert_eq!(GameResult::Win, PlayerRole::Scissors.compare(&PlayerRole::Lizard));
                assert_eq!(GameResult::Win, PlayerRole::Lizard.compare(&PlayerRole::Spock));
                assert_eq!(GameResult::Win, PlayerRole::Lizard.compare(&PlayerRole::Paper));
                assert_eq!(GameResult::Win, PlayerRole::Spock.compare(&PlayerRole::Scissors));
                assert_eq!(GameResult::Win, PlayerRole::Spock.compare(&PlayerRole::Rock));
            }

            #[test]
            fn when_losing_role_does_comparison_then_lose() {
                assert_eq!(GameResult::Lose, PlayerRole::Rock.compare(&PlayerRole::Paper));
                assert_eq!(GameResult::Lose, PlayerRole::Rock.compare(&PlayerRole::Spock));
                assert_eq!(GameResult::Lose, PlayerRole::Paper.compare(&PlayerRole::Scissors));
                assert_eq!(GameResult::Lose, PlayerRole::Paper.compare(&PlayerRole::Lizard));
                assert_eq!(GameResult::Lose, PlayerRole::Scissors.compare(&PlayerRole::Rock));
                assert_eq!(GameResult::Lose, PlayerRole::Scissors.compare(&PlayerRole::Spock));
                assert_eq!(GameResult::Lose, PlayerRole::Lizard.compare(&PlayerRole::Rock));
                assert_eq!(GameResult::Lose, PlayerRole::Lizard.compare(&PlayerRole::Scissors));
                assert_eq!(GameResult::Lose, PlayerRole::Spock.compare(&PlayerRole::Lizard));
                assert_eq!(GameResult::Lose, PlayerRole::Spock.compare(&PlayerRole::Paper));
            }

            #[test]
            fn test_enum_display() {
                assert_eq!("ROCK", format!("{}", PlayerRole::Rock));
                assert_eq!("PAPER", format!("{}", PlayerRole::Paper));
                assert_eq!("SCISSORS", format!("{}", PlayerRole::Scissors));
                assert_eq!("LIZARD", format!("{}", PlayerRole::Lizard));
                assert_eq!("SPOCK", format!("{}", PlayerRole::Spock));
            }

            #[test]
            fn when_enum_retrieved_by_name_then_okay() {
                if let Ok(PlayerRole::Rock) = PlayerRole::from_str("rock") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Rock) = PlayerRole::from_str("ROCK") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Rock) = PlayerRole::from_str("RoCk") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Paper) = PlayerRole::from_str("paper") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Paper) = PlayerRole::from_str("PAPER") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Paper) = PlayerRole::from_str("pApEr") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Scissors) = PlayerRole::from_str("scissors") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Scissors) = PlayerRole::from_str("SCISSORS") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Scissors) = PlayerRole::from_str("sCISsOrs") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Lizard) = PlayerRole::from_str("lizard") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Lizard) = PlayerRole::from_str("LIZARD") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Lizard) = PlayerRole::from_str("Lizard") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Spock) = PlayerRole::from_str("spock") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Spock) = PlayerRole::from_str("SPOCK") { } else { panic!("Expecting different PlayerRole"); }
                if let Ok(PlayerRole::Spock) = PlayerRole::from_str("spOck") { } else { panic!("Expecting different PlayerRole"); }
            }

            #[test]
            fn do_some_random_role_selection() {
                const SAMPLE_COUNT: i32 = 50000;
                const EXPECTED_MEAN_VALUE: i32 = SAMPLE_COUNT / 5;
                const VARIANCE: i32 = EXPECTED_MEAN_VALUE * 2 / 100;

                let mut rng = rand::thread_rng();

                let mut histogram:HashMap<PlayerRole, i32> = HashMap::new();
                histogram.insert(PlayerRole::Rock, 0);
                histogram.insert(PlayerRole::Paper, 0);
                histogram.insert(PlayerRole::Scissors, 0);
                histogram.insert(PlayerRole::Lizard, 0);
                histogram.insert(PlayerRole::Spock, 0);

                for _idx in 0..SAMPLE_COUNT {
                    let role: PlayerRole = rng.gen();
                    histogram.entry(role).and_modify(|e| { *e += 1 });
                }

                for (key, val) in histogram.iter() {
                    assert!(EXPECTED_MEAN_VALUE - VARIANCE <= *val, "{} count is out of range", key);
                    assert!(*val <= EXPECTED_MEAN_VALUE + VARIANCE, "{} count is out of range", key)
                }
            }
        }
    }


}