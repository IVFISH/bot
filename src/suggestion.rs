use crate::piece::*;
use crate::placement::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub piece: Piece,
}

impl Suggestion {
    pub fn new(piece: Piece) -> Self {
        Self { piece }
    }
}
