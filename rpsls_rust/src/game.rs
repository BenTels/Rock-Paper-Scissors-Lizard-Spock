mod game {

    mod game_result {
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

    mod game_player_role {
        use std::fmt::{Display, Formatter};
        use super::game_result::GameResult;

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

            fn compare(&self, other: &PlayerRole) -> GameResult {
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

        #[cfg(test)]
        mod tests {
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
        }
    }


}