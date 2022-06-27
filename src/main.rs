use std::io; // for I/O
use std::io::Write; // for flushing stdout

mod debug_boards; // extra starting board layouts for debugging purposes

fn main() {
    let mut board = MancalaBoard::new();
    // let mut board = MancalaBoard::new_visual_debug();   //DEBUG
    let mut game_state = MoveResult::Continuing(Player::P1); // P1 takes first turn
    // TODO make this conditional less verbose if possible?
    while game_state == MoveResult::Continuing(Player::P1) ||
        game_state == MoveResult::Continuing(Player::P2) {
        let current_player = board.turn.to_owned();
        draw_board(&board, &current_player); // paint the TUI

        // gets user input for which pit to move, and retries if an error occurs
        let move_input: Move = loop {
            print!("Choose a pit to move: ");
            io::stdout().flush().expect("ERROR: Failed to flush stdout"); // flush stdout so input is on same line
            let mut input = String::new(); // creates input variable
            //read command-line input
            io::stdin()
                .read_line(&mut input) // reads input
                .expect("ERROR: Failed to read line."); // panics with message if error occurs
            // match and compare input to convert to Move (trim newline)
            // assigns to Some(Move) if valid input is received, otherwise returns None
            let move_input = match input.as_str().trim() {
                "A" | "a" | "1" => Some(Move::A),
                "B" | "b" | "2" => Some(Move::B),
                "C" | "c" | "3" => Some(Move::C),
                "D" | "d" | "4" => Some(Move::D),
                "E" | "e" | "5" => Some(Move::E),
                "F" | "f" | "6" => Some(Move::F),
                _ => None,
            };
            if move_input.is_some() {
                break move_input.unwrap();
            } else {
                println!("Invalid input! Enter A,B,C,D,E, or F.\n");
            }
        };

        println!("move_input: Move::{:?}", move_input); //DEBUG
        game_state = board.update(&move_input);
        draw_board(&board, &current_player); // update the TUI

        // println!("DEBUG: game_state: {:?}", game_state);
        print!("Press ENTER to end your turn. ");
        io::stdout().flush().expect("ERROR: Failed to flush stdout"); // flush stdout so input is on same line
        let _ = io::stdin().read_line(&mut String::from("")).unwrap();
    }

}

// used to identify a player. Variants indicate Player One (P1) or Player Two (P2)
#[derive(PartialEq,Debug,Clone,Copy)]
enum Player {
    P1,
    P2,
}

// The state of the game as the result of a move
#[derive(PartialEq,Debug)]
enum MoveResult {
    Won(Player),    // which player has won
    Continuing(Player), // which player is up next (can be same player in case of bonus turns)
    Draw,   // game is over and scores are tied
    Invalid,    // last move attempt was invalid (empty pit selected)
}

// which pit the player has selected to move
#[derive(Debug)]
enum Move {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct MancalaBoard {
    p1_board: [u32; 6],
    p2_board: [u32; 6],
    p1_store: u32,
    p2_store: u32,
    turn: Player,
}

impl MancalaBoard {
    /// Creates a new MancalaBoard with its values initialized to a new game layout according to
    /// the game rules of mancala.
    pub fn new() -> Self {
        Self {
            p1_board: [4, 4, 4, 4, 4, 4],
            p2_board: [4, 4, 4, 4, 4, 4],
            p1_store: 0,
            p2_store: 0,
            turn: Player::P1,
        }
    }
    /// Move a pit's contents and update board data
    /// # Arguments
    /// * `self` - the MancalaBoard to update
    /// * `move_pit` - which pit the player has chosen to move
    /// * `player` - the player who is making the move
    pub fn update(self: &mut Self, move_pit: &Move) -> MoveResult {
        // TODO finish this (work in progress)

        // map board data to relative variables depending on the current player
        let row_player;
        let row_opponent;
        let store_player;
        match self.turn {
            Player::P1 => {
                row_player = &mut self.p1_board;
                row_opponent = &mut self.p2_board;
                store_player = &mut self.p1_store;
            },
            Player::P2 => {
                row_player = &mut self.p2_board;
                row_opponent = &mut self.p1_board;
                store_player = &mut self.p2_store;
            },
        }

        let pit_pos = match move_pit {
            Move::A => 0,
            Move::B => 1,
            Move::C => 2,
            Move::D => 3,
            Move::E => 4,
            Move::F => 5,
        };
        if row_player[pit_pos] == 0 { return MoveResult::Invalid; }    // invalid move if selected pit is empty
        // empty the pit's contents into a "hand" to redistribute
        let mut hand = row_player[pit_pos];
        row_player[pit_pos] = 0;
        // which position in player's row is the final piece dropped (None if not in player's row)
        let mut end_home_pos: Option<usize> = None;

        // distribute hand across player's row, starting at the pit to the right of the chosen pit
        for (index, pit) in row_player.iter_mut().enumerate() {
            if hand == 0 {
                end_home_pos = Some(index);
                break;
            }
            if index <= pit_pos {continue}   // skip this step if selected pit is to the right of current i value
            *pit += 1;
            hand -= 1;
        }
        // distribute remainder of hand around the board, depositing a piece in the
        // move-making player's store at the appropriate point
        'outer: loop {
            if hand > 0 {
                *store_player += 1;
                hand -= 1;
            } else {break 'outer}
            for pit in row_opponent.iter_mut() {
                if hand == 0 {break 'outer}
                *pit += 1;
                hand -= 1;
            }
            for (index, pit) in row_player.iter_mut().enumerate() {
                if hand == 0 {
                    end_home_pos = Some(index);
                    break 'outer;
                }
                *pit += 1;
                hand -= 1;
            }
        }
        // TODO check for bonus turns
        // TODO check for capturing

        // check for game over
        if MancalaBoard::is_row_empty(row_player) || MancalaBoard::is_row_empty(row_opponent) {
            // award remaining pieces and tally scores, then return MoveResult::Won(Player)
            println!("DEBUG: GAME OVER!");
            return MoveResult::Won(self.turn);
        }
        // if continuing to next player's turn:
        // (this is sloppy shortcut code)
        if self.turn == Player::P1 {
            self.turn = Player::P2;
            return MoveResult::Continuing(Player::P2);
        } else {
            self.turn = Player::P1;
            return MoveResult::Continuing(Player::P1);
        }

    }
    fn is_row_empty(row: &[u32]) -> bool {
        for i in row.iter() {
            if *i != 0 { return false; }
        }
        true
    }
    // returns index number of pit opposite in opponent's row
    fn opposite_pit_pos(pit: &usize) -> usize {
        5 - pit
    }
}

// paints the board in TUI
// board: the MancalaBoard to display
// perspective: from which player's visual perspective the board is shown
fn draw_board(board: &MancalaBoard, perspective: &Player) {
    // map components of board to variables to correctly depict player perspective
    let row_near;
    let row_far;
    let store_right;
    let store_left;
    let player_name; // string representation of the current player
    match perspective {
        Player::P1 => {
            row_near = board.p1_board;
            row_far = board.p2_board;
            store_right = board.p1_store;
            store_left = board.p2_store;
            player_name = String::from("Player One");
        },
        Player::P2 => {
            row_near = board.p2_board;
            row_far = board.p1_board;
            store_right = board.p2_store;
            store_left = board.p1_store;
            player_name = String::from("Player Two");
        },
    }

    clearscreen::clear().unwrap(); // clear screen

    println!("/--*--*--*--*--*--*--*--*--*--*--*--*--*--*--*--\\");
    println!("| . . . . . .  M  A  N  C  A  L  A  . . . . . . |");
    println!("\\--*--*--*--*--*--*--*--*--*--*--*--*--*--*--*--/");
    println!();
    println!("{}'s turn:", player_name);
    println!();

    //print opposite player's side (in reverse because it's the opposite side of the board)
    print!("[    ] ");
    for i in row_far.iter().rev() {
        let pit = i;
        let pit = pad_number(*pit);
        print!("({}) ", pit)
    }
    print!("[    ] \n");

    //print middle gap + stores
    let store_left = pad_number(store_left);
    let store_right = pad_number(store_right);
    println!("[ {} ]                               [ {} ]", store_left, store_right);

    //print current player's side
    print!("[    ] ");
    for i in row_near {
        let pit = i;
        let pit = pad_number(pit);
        print!("({}) ", pit)
    }
    print!("[    ] \n");

    //print bottom (board labels)
    println!("         A    B    C    D    E    F   STORE");
    println!();
}

// converts a number to a string and
// adds a space to the beginning if the number is a single digit.
fn pad_number(num: u32) -> String {
    if num > 9 {
        return num.to_string();
    }
    else {
        return " ".to_string() + &num.to_string();
    }
}
