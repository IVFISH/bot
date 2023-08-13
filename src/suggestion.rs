use crate::piece::*;
use crate::board::*;
use crate::placement::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub info: String
}

impl Suggestion {
    pub fn new(board: Board) -> Self {
        Self { 
            info: serde_json::to_string(&board.arr).unwrap(),
        }
    }
}
