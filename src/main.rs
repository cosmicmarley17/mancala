fn main() {
    let mut winner = 0;
    let mut board = MancalaBoard::new();
    // Uncomment loop when done debugging
    // loop {
    //     play_turn(Turn::P1, &mut board);
    //     play_turn(Turn::P2, &mut board);
    // }
    play_turn(Turn::P1, &mut board);    //DEBUG
}

// player whose turn it is
enum Turn {
    P1,
    P2,
}

// which pit the player has selected to move
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

fn play_turn(player: Turn, board: &mut MancalaBoard) {
    draw_board(&board); // paint the TUI

    // get player input for move here
    let mut move_input: Move;

    if check_gameover() {
        //distribute leftover points, count store totals, and declare winner
        println!("TODO: write end-of-game logic");
    }
}

// paints the board in TUI
fn draw_board(board: &MancalaBoard) {
    // TODO add a function param for current player, so the opposite side is drawn accordingly
    // rather than hardcoded from player 1's POV

    println!("TODO: write draw_board()");
    println!("");
    //print opposite player's side
    // println!("[    ] ({}) ({}) ({}) ({}) ({}) ({}) [    ]", );
    print!("[    ] ");
    for i in board.p2_board {
        let pit = i;
        let pit = pad_number(pit);
        print!("({}) ", pit)
    }
    print!("[    ] \n");

    //print middle gap + stores
    let store_left = pad_number(board.p2_store);
    let store_right = pad_number(board.p1_store);
    println!("[ {} ]                               [ {} ]", store_left, store_right);

    //print current player's side
    // println!("[    ] ({}) ({}) ({}) ({}) ({}) ({}) [    ]");

    //print bottom (board labels)
    // println!("         A    B    C    D    E    F   STORE");

    // println!("{:?}", board.p1_board[2]);
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
