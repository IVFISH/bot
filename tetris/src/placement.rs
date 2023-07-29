use crate::command::Command;
use crate::piece::Piece;

pub struct Placement<'a> {
    pub piece: Piece,

    // references to the source for generating the move
    pub trivial_base: &'a Vec<Command>,
    pub nontrivial_extension: &'a Vec<Command>,
    pub nontrivial_index: usize,
}
