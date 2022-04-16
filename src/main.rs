use std::io; // for I/O
use std::io::Write; // for flushing stdout

fn main() {
    let mut winner = 0; //0 if no winner, 1 if player one, 2 if player 2
    let mut board = MancalaBoard::new();
    // Uncomment loop when done debugging
    // loop {
    //     play_turn(Turn::P1, &mut board, &mut winner);
    //     play_turn(Turn::P2, &mut board, &mut winner);
    // }
    play_turn(Turn::P1, &mut board, &mut winner);    //DEBUG
}

// player whose turn it is
// hmm do I need to make this a more general
enum Turn {
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
    pub fn new() -> Self {
        Self {
            p1_board: [4, 4, 4, 4, 4, 4],
            p2_board: [4, 4, 4, 4, 4, 4],
            p1_store: 0,
            p2_store: 0,
        }
    }
}

fn play_turn(player: Turn, board: &mut MancalaBoard, winner: &mut u32) {
    draw_board(&board); // paint the TUI

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

    //TODO Need to check if player gains an extra turn, use another loop

    // check if this turn ends the game
    if check_gameover() {
        //distribute leftover points, count store totals, and declare winner
        println!("TODO: write end-of-game logic");
    }
}

// paints the board in TUI
fn draw_board(board: &MancalaBoard) {
    // TODO add a function param for current player, so the opposite side is drawn accordingly
    // rather than hardcoded from player 1's POV

    clearscreen::clear().unwrap(); // clear screen

    //ascii text art
    println!("    __  __                       _       ");
    println!("   |  \\/  |                     | |      ");
    println!("   | \\  / | __ _ _ __   ___ __ _| | __ _ ");
    println!("   | |\\/| |/ _` | '_ \\ / __/ _` | |/ _` |");
    println!("   | |  | | (_| | | | | (_| (_| | | (_| |");
    println!("   |_|  |_|\\__,_|_| |_|\\___\\__,_|_|\\__,_|");
    println!("--*--*--*--*--*--*--*--*--*--*--*--*--*--*--");
    println!();

    //print opposite player's side (in reverse because it's the opposite side of the board)
    print!("[    ] ");
    for i in board.p2_board.iter().rev() {
        let pit = i;
        let pit = pad_number(*pit);
        print!("({}) ", pit)
    }
    print!("[    ] \n");

    //print middle gap + stores
    let store_left = pad_number(board.p2_store);
    let store_right = pad_number(board.p1_store);
    println!("[ {} ]                               [ {} ]", store_left, store_right);

    //print current player's side
    print!("[    ] ");
    for i in board.p1_board {
        let pit = i;
        let pit = pad_number(pit);
        print!("({}) ", pit)
    }
    print!("[    ] \n");

    //print bottom (board labels)
    println!("         A    B    C    D    E    F   STORE");
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
