use thiserror::Error;

use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum GreedError {
  #[error("Invalid direction")]
  InvalidDirection,
  #[error("Bad move")]
  BadMove,
  #[error("Game complete")]
  GameComplete,
}

#[derive(Error, Debug, PartialEq)]
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
  #[error("Invalid meta data")]
  InvalidMetaData { cause: json5::Error },
  #[error("No trailing new line")]
  NoTrailingNewLine,
}

#[derive(Error, Debug, PartialEq)]
#[error("Invalid character ({found:})")]
pub struct TileParseError {
  pub found: char,
}

#[derive(Error, Debug, PartialEq)]
pub enum InternalGreedError {
  #[error("Index out of bounds")]
  IndexOutOfBounds,
  #[error("Internal greed error")]
  OhNo,
}
