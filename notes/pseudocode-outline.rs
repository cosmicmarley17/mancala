fn main() {
    mancala();
}

fn play_turn(u32: player) {
    // get input to choose a pit to move
    // move the pit and do the math, updating the array values and scores
    draw_board(); // paint the UI
    if check_gameover() {
        //distribute leftover points and count points, then declare winner
        winner = player;
    }
}

fn mancala() {
    let mut winner;
    loop {
        play_turn(1);
        play_turn(2);
    }
}

fn draw_board() {}

fn check_gameover() {
    //checks if the game is over, i.e. if one side of the board is clear.
    //returns boolean true if game is over
}
