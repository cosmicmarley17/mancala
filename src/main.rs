fn main() {
    let mut winner;
    let mut p1_board = [4,4,4,4,4,4,0];
    let mut p2_board = [4,4,4,4,4,4,0];
    loop {
        //TODO consider combining p1 and p2 boards into a unified board struct
        //  - would the stores become their own data structures?
        play_turn(Turn::P1, &mut p1_board, &mut p2_board);
        play_turn(Turn::P2, &mut p1_board, &mut p2_board);
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

fn play_turn(player: Turn, p1_board: &mut [u32;7], p2_board: &mut [u32;7]) {
    draw_board();   // paint the TUI

    // get player input for move here
    let mut move_input: Move;

    if check_gameover() {
        //distribute leftover points, count store totals, and declare winner
    }
}

// paints the board in TUI
fn draw_board(p1_board: &[u32;7], p2_board: &[u32;7]) {

}
