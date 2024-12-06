use super::{GameStateRebuildFromDiffError, Pos};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlayableError {
  #[error("Invalid direction")]
  InvalidDirection,
  #[error("Bad move")]
  BadMove,
  #[error("Tried to undo an invalid move (probably originating from a corrupted save file)")]
  UndoInvalidMove,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GameFieldParserError {
  #[error("Player not found on the game field")]
  PlayerNotFound,
  #[error("Multiple players found on the game field ({first} and {second})")]
  AmbiguousPlayer { first: Pos, second: Pos },
  #[error("Game field contains an invalid character ({found}) at {pos}")]
  InvalidCharacter { found: char, pos: Pos },
  #[error("Game field not rectangular")]
  NotRectangular,
  #[error("Game field contains empty lines! THIS IS NOT OK!")]
  EmptyLine,
  #[error("No trailing new line")]
  NoTrailingNewLine,
  #[error("Invalid size")]
  InvalidSize,
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("Invalid character ({found})")]
pub struct TileParseError {
  pub found: char,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct JsonErrorWrapper {
  #[allow(dead_code)]
  #[from]
  source: serde_json::Error,
}

impl PartialEq for JsonErrorWrapper {
  fn eq(&self, _other: &Self) -> bool {
    true // JsonErrors are always Eq, because serde_json::Error doesn't implement PartialEq, Eq
  }
}
impl Eq for JsonErrorWrapper {}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GreedParserError {
  #[error("Empty string")]
  EmptyString,
  #[error("Could not determine the initial game field. Provide at least one of: seed, initial_game_field or last_game_field")]
  MissingGameFieldInformation,
  #[error("Invalid meta data format")]
  InvalidMetaDataFromat {
    #[from]
    source: JsonErrorWrapper,
  },
  #[error("Failed to parse game field")]
  GameFieldParserError {
    #[from]
    source: GameFieldParserError,
  },
  #[error("Failed to rebuild game state from initial_game_field and last_game_field")]
  GameStateRebuildFromDiffError {
    #[from]
    source: GameStateRebuildFromDiffError,
  },
  #[error("Failed to rebuild game state from initial_game_field and moves array")]
  GameStateRebuildFromMovesError {
    #[from]
    source: PlayableError,
  },
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveValidationError {
  #[error("Move {move_number} is invalid")]
  InvalidMove { move_number: usize },
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ReproductionError {
  #[error("Invalid move")]
  MoveValidationError {
    #[from]
    source: MoveValidationError,
  },
  #[error("Seed does not match game field")]
  WrongSeed,
}
