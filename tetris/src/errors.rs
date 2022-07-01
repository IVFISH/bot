use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GameError {
    NotInBounds,
    Collision,
    TopOut,
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::NotInBounds => write!(f, "Not In Bounds")?,
            GameError::Collision => write!(f, "Collision")?,
            GameError::TopOut => write!(f, "Thank you for playing.")?,
        }
        Ok(())
    }
}

impl Error for GameError {}
