fn main() {
    let mut winner = 0;
    let mut board = MancalaBoard::new();
    loop {
        play_turn(Turn::P1, &mut board);
        play_turn(Turn::P2, &mut board);
    }
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
    println!("TODO: write draw_board()");
}

fn check_gameover() -> bool {
    println!("TODO: write check_gameover()");
    false
}
