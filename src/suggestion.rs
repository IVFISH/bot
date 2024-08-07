use crate::board::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub info: String,
}

impl Suggestion {
    pub fn new(board: Board) -> Self {
        Self {
            info: serde_json::to_string(&board.arr).unwrap(),
        }
    }
}
