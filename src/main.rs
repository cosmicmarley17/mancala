use std::io; // for I/O
use std::io::Write; // for flushing stdout

fn main() {
    let mut winner = 0; //0 if no winner, 1 if player one, 2 if player 2
    // let mut board = MancalaBoard::new();
    let mut board = MancalaBoard::new_visual_debug();   //DEBUG
    // Uncomment loop when done debugging
    // loop {
    //     play_turn(Player::P1, &mut board, &mut winner);
    //     play_turn(Player::P2, &mut board, &mut winner);
    // }
    play_turn(Player::P1, &mut board, &mut winner);    //DEBUG
}

// used to identify a player. Variants indicate Player One (P1) or Player Two (P2)
enum Player {
    P1,
    P2,
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
}

impl MancalaBoard {
    // starting board layout
    pub fn new() -> Self {
        Self {
            p1_board: [4, 4, 4, 4, 4, 4],
            p2_board: [4, 4, 4, 4, 4, 4],
            p1_store: 0,
            p2_store: 0,
        }
    }
    pub fn new_visual_debug() -> Self {
        Self {
            p1_board: [1, 2, 3, 4, 5, 6],
            p2_board: [7, 8, 9, 10, 11, 12],
            p1_store: 1,
            p2_store: 2,
        }
    }
    // move a pit and update numbers
    pub fn update(self: &mut Self, move_pit: &Move, player: &Player) -> &mut Self {
        // TODO finish this (work in progress)
        // TODO match statement for current player
        let pit_pos = match move_pit {
            Move::A => 0,
            Move::B => 1,
            Move::C => 2,
            Move::D => 3,
            Move::E => 4,
            Move::F => 5,
        };
        return self;
    }
}

fn play_turn(current_player: Player, mut board: &mut MancalaBoard, winner: &mut u32) {
    loop {
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
                println!("Invalid move! Enter A,B,C,D,E, or F.\n");
            }
        };
        println!("move_input: Move::{:?}", move_input); //DEBUG
        board = board.update(&move_input, &current_player);


        // check if this turn ends the game
        if check_gameover() {
            //distribute leftover points, count store totals, and declare winner
            println!("TODO: write end-of-game logic");
        }

        //TODO Need to check if player earns an extra turn (continue loop)
        let turn_end: bool;
        turn_end = true;
        if turn_end {break} // breaks turn loop if turn is over
    }
}

// paints the board in TUI
fn draw_board(board: &MancalaBoard, current_player: &Player) {
    // map components of board to variables to correctly depict player perspective
    let row_near;
    let row_far;
    let store_right;
    let store_left;
    let player_name; // string representation of the current player
    match current_player {
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

fn check_gameover() -> bool {
    //TODO: write check_gameover()
    false
}
