use thiserror::Error;

use super::*;

#[derive(Error, Debug)]
pub enum GreedError {
  #[error("Invalid direction")]
  InvalidDirection,
}

#[derive(Error, Debug)]
pub enum GameFieldParserError {
  #[error("Player not found on the game field")]
  PlayerNotFound,
  #[error("Multiple players found on the game field")]
  AmbiguousPlayer,
  #[error("Game field contains an invalid character ({found:}) at {pos:}")]
  InvalidCharacter { found: char, pos: Pos },
  #[error("Game field not rectangular")]
  NotRectangular,
  #[error("Game field contains empty lines! THIS IS NOT OK!")]
  EmptyLine,
}

#[derive(Error, Debug)]
#[error("Invalid character ({found:})")]
pub struct TileParseError {
  found: char,
}

#[derive(Error, Debug)]
pub enum InternalGreedError {
  #[error("Index out of bounds")]
  IndexOutOfBounds,
  #[error("Internal greed error")]
  OhNo,
}
