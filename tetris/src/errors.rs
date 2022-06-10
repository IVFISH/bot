use std::{error::Error};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GameError {
    NotInBounds,
    TopOut
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for GameError {}
