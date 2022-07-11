use super::Size2D;
use std::fmt::Write;
use thiserror::Error;

pub type TileProbs = [u8; 9];

pub const DEFAULT_TILE_PROBABILITIES: TileProbs = [1, 1, 1, 1, 1, 1, 1, 1, 1];

/// TODO: Maybe rename to user str error?
#[derive(Error, Debug, PartialEq)]
pub enum SeedCreationError {
  #[error("Unexpected char {char} these are valid: 'A-Za-z0-9_'")]
  InvalidCharacter { char: char },
}

#[derive(Error, Debug, PartialEq)]
pub enum SeedConversionError {
  #[error("Unexpected char {char} these are valid: 'A-Za-z0-9_'")]
  InvalidCharacter { char: char },
  #[error("Empty string")]
  EmptyString,
  #[error("Dimension format error <x_size>x<y_size>")]
  InvalidDimension,
  #[error("Invalid probabilities")]
  InvalidProbabilities,
  #[error("Unexpected hash tag")]
  UnexpectedHashTag,
}

/// # Seed format yummy:
///
/// The seed encodes the user_str, the dimensions and optionally the probabilities for all tiles.
/// The dimensions and probabilies are encoded as ~upper_alternating_case~ hex.
///
/// \# is used as a separator
///
/// <> is a placeholder
///
/// [] indicates optional
///
/// Format: <user_str>#<x_size>x<y_size>[#112233445566778899]
///
/// Representation:
/// * `user_str: A-Za-z0-9_`
/// * `x_size: unsigned`
/// * `y_size: unsigned`
/// * \<T>`XX: probability of tile T as two hex digits` where `T is the tile number in 1..=9`
pub struct Seed {
  tile_probabilities: TileProbs,
  size: Size2D,
  user_str: String,
}

impl Seed {
  /// With `DEFAULT_TILE_PROBABILITIES`
  pub fn new(user_str: String, size: Size2D) -> Self {
    // maybe SeedBuilder or maybe ValidatedUserString type that checks the user str? then the return can stay a Self?
    todo!()
  }
  pub fn new_with_probabilities(
    user_str: String,
    size: Size2D,
    tile_probabilities: TileProbs,
  ) -> Self {
    todo!()
  }
  /// With `DEFAULT_TILE_PROBABILITIES`
  pub fn new_random(size: Size2D) -> Self {
    todo!()
  }
  pub fn new_random_with_probabilities(size: Size2D, tile_probabilities: TileProbs) -> Self {
    todo!()
  }
}

impl From<Seed> for String {
  fn from(seed: Seed) -> Self {
    let Seed {
      size: Size2D { x_size, y_size },
      user_str,
      tile_probabilities,
    } = seed;
    let mut str = format!("{user_str}#{x_size:x}x{y_size:x}");
    if tile_probabilities != DEFAULT_TILE_PROBABILITIES {
      str.push('#');
      for prob in tile_probabilities {
        write!(str, "{prob:02x}");
      }
    }
    str
  }
}

impl TryFrom<String> for Seed {
  type Error = SeedConversionError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    todo!()
  }
}
