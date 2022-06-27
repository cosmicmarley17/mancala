use crate::{MancalaBoard,Player};

impl MancalaBoard {
    pub fn new_visual_debug() -> Self {
        Self {
            p1_board: [1, 2, 3, 4, 5, 6],
            p2_board: [7, 8, 9, 10, 11, 12],
            p1_store: 1,
            p2_store: 2,
            turn: Player::P1,
        }
    }
}
