use super::{GameStateRebuildFromDiffError, Pos};
use std::num::TryFromIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PlayableError {
  #[error("Invalid direction")]
  InvalidDirection,
  #[error("Bad move")]
  BadMove,
  #[error("Tried to undo an invalid move (probably originating from a corrupted save file)")]
  UndoInvalidMove,
}

#[derive(Error, Debug, PartialEq)]
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

#[derive(Error, Debug, PartialEq)]
#[error("Invalid character ({found})")]
pub struct TileParseError {
  pub found: char,
}

#[derive(Error, Debug, PartialEq)]
pub enum GreedParserError {
  #[error("Empty string")]
  EmptyString,
  #[error("Could not determine the initial game field. Provide at least one of: seed, initial_game_field or last_game_field")]
  MissingGameFieldInformation,
  #[error("GameMeta contains moves but no initial game_field")]
  MovesButNoInitialGameField,
  #[error("Invalid meta data format")]
  InvalidMetaDataFromat { cause: json5::Error },
  #[error("Invalid duration")]
  InvalidDuration { cause: TryFromIntError },
  #[error("Failed to parse game field")]
  GameFieldParserError { cause: GameFieldParserError },
  #[error("Failed to rebuild game state from initial_game_field and last_game_field")]
  GameStateRebuildFromDiffError {
    cause: GameStateRebuildFromDiffError,
  },
  #[error("Failed to rebuild game state from initial_game_field and moves array")]
  GameStateRebuildFromMovesError { cause: PlayableError },
}

impl From<GameStateRebuildFromDiffError> for GreedParserError {
  fn from(cause: GameStateRebuildFromDiffError) -> Self {
    GreedParserError::GameStateRebuildFromDiffError { cause }
  }
}

impl From<PlayableError> for GreedParserError {
  fn from(cause: PlayableError) -> Self {
    GreedParserError::GameStateRebuildFromMovesError { cause }
  }
}

#[derive(Error, Debug, PartialEq)]
pub enum MoveValidationError {
  #[error("Move {move_number} is invalid")]
  InvalidMove { move_number: usize },
}

#[derive(Error, Debug, PartialEq)]
pub enum ReproductionError {
  #[error("Invalid move")]
  MoveValidationError { cause: MoveValidationError },
  #[error("Seed does not match game field")]
  WrongSeed,
}
