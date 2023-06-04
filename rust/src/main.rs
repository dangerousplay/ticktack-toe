use std::fmt::{Display, Formatter, Write};
use std::io::Read;

use itertools::Itertools;


const WINNER_POSITIONS: [[usize; 3]; 8] = [
    // Horizontal Lines
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    // Vertical Lines
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
    // Diagonal Lines
    [1, 5, 9],
    [3, 5, 7],
];

const ROW: usize = 3;

pub type SlotValue = Option<Player>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => f.write_str("X"),
            Player::O => f.write_str("O"),
        }
    }
}

pub struct GameState {
    board: [SlotValue; 9],
    current_player: Player,
}

pub enum GameError {
    SlotTaken,
    InvalidSlot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Winner {
    Player(Player),
    Tie,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: Default::default(),
            current_player: Player::X,
        }
    }
}

pub struct Game {
    state: GameState,
}

fn is_all_same<E: Eq>(slice: &Vec<E>) -> bool {
    slice
        .get(0)
        .map(|first| slice.iter().all(|x| x == first))
        .unwrap_or(true)
}

fn display_slot(slot: &SlotValue) -> String {
    match slot {
        Some(p) => format!("{}", p),
        None => " ".to_string(),
    }
}

impl Game {
    pub fn start(&mut self) {
        let mut stdin = std::io::stdin();

        println!("Starting tic tac toe game");

        loop {
            println!("Player {}", self.state.current_player);
            println!("Type the slot [1-9]: ");

            let mut input = String::new();

            if let Err(e) = stdin.read_line(&mut input) {
                eprintln!("Failed to read input: {}", e);
                return;
            }

            let slot = input.trim_end().parse::<usize>();

            let slot = match slot {
                Err(e) => {
                    eprintln!("Invalid slot: {}", e);
                    continue;
                }
                Ok(e) => e,
            };

            if let Err(e) = self.run_turn(slot) {
                match e {
                    GameError::SlotTaken => {
                        eprintln!("Slot already taken, choose another one");
                        continue;
                    }
                    GameError::InvalidSlot => {
                        eprintln!("Invalid slot informed");
                        continue;
                    }
                }
            }

            self.display_board();

            println!();

            if let Some(w) = self.check_winner() {
                match w {
                    Winner::Player(p) => {
                        println!("Player {} won", p);
                    }
                    Winner::Tie => { println!("It's a tie")}
                }
                break;
            }
        }
    }

    fn display_board(&self) {
        let board = self.state.board.iter()
            .chunks(ROW)
            .into_iter()
            .map(|slots| {
                slots.map(display_slot).join(" | ")
            }).join("\n----------\n");

        println!("{}", board);
    }

    fn check_winner(&self) -> Option<Winner> {
        let winner = WINNER_POSITIONS
            .iter()
            .map(|pos| {
                pos.iter()
                    .filter_map(|x| self.state.board[x.clone() - 1].clone())
                    .collect::<Vec<_>>()
            })
            .filter_map(|x| {
                if x.len() == ROW && is_all_same(&x) {
                    Some(x[0].clone())
                } else {
                    None
                }
            }).next();

        match winner {
            None => {
                if self.state.board.iter().filter(|x| x.is_some()).count() == self.state.board.len()
                {
                    Some(Winner::Tie)
                } else {
                    None
                }
            }
            Some(e) => Some(Winner::Player(e)),
        }
    }

    fn run_turn(&mut self, slot: usize) -> Result<(), GameError> {
        if !(1..=9).contains(&slot) {
            return Err(GameError::InvalidSlot);
        }

        let real_slot = slot - 1;

        if self.state.board[real_slot.clone()].is_some() {
            return Err(GameError::SlotTaken);
        }

        self.state.board[real_slot] = Some(self.state.current_player.clone());

        Game::next_player(&mut self.state);

        Ok(())
    }

    fn next_player(state: &mut GameState) {
        let next_player = match state.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        state.current_player = next_player;
    }
}

fn main() {
    let state = GameState::new();
    let mut game = Game { state };

    game.start();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_winner_horizontal() {
        let game_state = GameState {
            board: [
                Some(Player::X), Some(Player::X), Some(Player::X),
                Some(Player::O), Some(Player::O), None,
                None,            None,            None,
            ],
            current_player: Player::X,
        };

        let mut game = Game { state: game_state };
        assert_eq!(game.check_winner(), Some(Winner::Player(Player::X)));

        game.state.board[2] = Some(Player::O);
        assert_eq!(game.check_winner(), None);
    }

    #[test]
    fn test_check_winner_vertical() {
        let game_state = GameState {
            board: [
                Some(Player::X), Some(Player::O), None,
                Some(Player::X), Some(Player::O), None,
                Some(Player::X), None,            None,
            ],
            current_player: Player::X,
        };
        let mut game = Game { state: game_state };

        assert_eq!(game.check_winner(), Some(Winner::Player(Player::X)));

        game.state.board[6] = Some(Player::O);
        assert_eq!(game.check_winner(), None);
    }

    #[test]
    fn test_check_winner_diagonal() {
        let game_state = GameState {
            board: [
                Some(Player::X), Some(Player::O), None,
                Some(Player::O), Some(Player::X), None,
                None,            None,            Some(Player::X),
            ],
            current_player: Player::O,
        };

        let game = Game { state: game_state };
        assert_eq!(game.check_winner(), Some(Winner::Player(Player::X)));
    }

    #[test]
    fn test_check_winner_tie() {
        let game_state = GameState {
            board: [
                Some(Player::X), Some(Player::O), Some(Player::X),
                Some(Player::O), Some(Player::X), Some(Player::O),
                Some(Player::O), Some(Player::X), Some(Player::O),
            ],
            current_player: Player::X,
        };

        let game = Game { state: game_state };
        assert_eq!(game.check_winner(), Some(Winner::Tie));
    }
}
