use super::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GreedError {
  #[error("Invalid direction")]
  InvalidDirection,
  #[error("Bad move")]
  BadMove,
  #[error("Tried to undo an invalid move (probably originating from a corrupted save file)")]
  UndoInvalidMove,
  #[error("Game complete")]
  GameComplete,
}

#[derive(Error, Debug, PartialEq)]
pub enum GameFieldParserError {
  #[error("Player not found on the game field")]
  PlayerNotFound,
  #[error("Multiple players found on the game field ({first:} and {second:})")]
  AmbiguousPlayer { first: Pos, second: Pos },
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
  #[error("Invalid size")]
  InvalidSize,
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
