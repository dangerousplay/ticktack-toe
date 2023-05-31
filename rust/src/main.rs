use std::io::Read;

pub type SlotValue = Option<Player>;

pub enum Player {
    X, O
}

pub struct GameState {
    board: [SlotValue; 9],
    current_player: Player
}

impl GameState {
    fn new() -> Self {
        Self {
            board: Default::default(),
            current_player: Player::X
        }
    }
}

pub struct Game {
    state: GameState
}

impl Game {
    pub fn start(&mut self) {
        let mut stdin = std::io::stdin();

        println!("Starting tic tac toe game");

        loop {
            let mut input = String::new();

            print!("Type the slot [1-9]: ");

            if let Err(e) = stdin.read_to_string(&mut input) {
                eprintln!("Failed to read input: {}", e);
                return;
            }
            
            let slot = input.parse::<u8>();
            
            let slot = match slot {
                Err(e) => {
                    eprintln!("Invalid slot: {}", e);
                    continue;
                },
                Ok(e) => e,
            };
        }
    }

    // fn run_turn(&mut self, slot: u8) -> Result<(), GameError> {
    //     self.state.board
    // }

    fn next_player(state: &mut GameState) {
        let next_player = match state.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        state.current_player = next_player;
    }

}

fn main() {
    let mut state = GameState::new();
    let mut game = Game { state };

    game.start();

    println!("Hello, world!");
}
