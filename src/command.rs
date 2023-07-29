#![allow(dead_code)]

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum Command {
    Null, // default
    MoveHorizontal(i8),
    MoveDrop,
    Rotate(u8),
    Backtrack(usize),
}

pub const COMMANDS: [Command; 6] = [
    Command::MoveHorizontal(1),
    Command::MoveHorizontal(2),
    Command::Rotate(1),
    Command::Rotate(2),
    Command::Rotate(3),
    Command::MoveDrop,
];
