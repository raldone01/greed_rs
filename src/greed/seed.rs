use super::{
  Size2D, Size2DConversionError, TileProbs, TileProbsConversionError, DEFAULT_SIZE,
  DEFAULT_TILE_PROBABILITIES,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UserStringError {
  #[error("Unexpected char {char} these are valid: 'A-Za-z0-9_'")]
  InvalidCharacter { char: char },
  #[error("UserString is empty")]
  Empty,
}

#[derive(Error, Debug, PartialEq)]
pub enum SeedConversionError {
  #[error("Invalid User String")]
  UserStringError { cause: UserStringError },
  #[error("Empty string")]
  EmptyString,
  #[error("Dimension format error expected: <x_size>x<y_size>")]
  InvalidDimension { cause: Size2DConversionError },
  #[error("Invalid probabilities")]
  InvalidProbabilities { cause: TileProbsConversionError },
  #[error("Unexpected hash tag")]
  UnexpectedHashTag,
  #[error("Unexpected end of the Seed")]
  UnexpectedEndOfSeed,
}

impl From<UserStringError> for SeedConversionError {
  fn from(cause: UserStringError) -> Self {
    SeedConversionError::UserStringError { cause }
  }
}

impl From<Size2DConversionError> for SeedConversionError {
  fn from(cause: Size2DConversionError) -> Self {
    SeedConversionError::InvalidDimension { cause }
  }
}

impl From<TileProbsConversionError> for SeedConversionError {
  fn from(cause: TileProbsConversionError) -> Self {
    SeedConversionError::InvalidProbabilities { cause }
  }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserString(String);

impl UserString {
  fn is_valid_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
  }
  fn validate_user_string(user_str: &str) -> Result<(), UserStringError> {
    if user_str.is_empty() {
      return Err(UserStringError::Empty);
    }
    user_str
      .chars()
      .find(|&char| !Self::is_valid_char(char))
      .map_or(Ok(()), |char| {
        Err(UserStringError::InvalidCharacter { char })
      })
  }
  const RANDOM_USER_STRING_LENGTH: usize = 16;
  const RANDOM_USER_STRING_DISTRIBUTION: Alphanumeric = Alphanumeric;
  pub fn as_str(&self) -> &str {
    &self.0[..]
  }
  pub fn new_random() -> Self {
    let inner = rand::thread_rng()
      .sample_iter(Self::RANDOM_USER_STRING_DISTRIBUTION)
      .map(|byte| byte as char)
      .take(Self::RANDOM_USER_STRING_LENGTH)
      .collect();

    Self(inner)
  }
}

impl From<UserString> for String {
  fn from(user_str: UserString) -> Self {
    user_str.0
  }
}
impl TryFrom<&str> for UserString {
  type Error = UserStringError;

  fn try_from(user_str: &str) -> Result<Self, Self::Error> {
    user_str
      .chars()
      .find(|&char| !Self::is_valid_char(char))
      .map_or(Ok(Self(String::from(user_str))), |char| {
        Err(UserStringError::InvalidCharacter { char })
      })
  }
}
impl Display for UserString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
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
/// Format: <user_str>[#<x_size>x<y_size>[#112233445566778899]]
///
/// Representation:
/// * `user_str: A-Za-z0-9_`
/// * `x_size: unsigned`
/// * `y_size: unsigned`
/// * \<T>`XX: probability of tile T as two hex digits` where `T is the tile number in 1..=9`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
pub struct Seed {
  tile_probabilities: TileProbs,
  size: Size2D,
  user_str: UserString,
}

impl Seed {
  /// tile_probabilities == None uses: `DEFAULT_TILE_PROBABILITIES`
  pub fn new(user_str: UserString, size: Size2D, tile_probabilities: Option<TileProbs>) -> Self {
    Self {
      tile_probabilities: tile_probabilities.unwrap_or(DEFAULT_TILE_PROBABILITIES),
      size,
      user_str,
    }
  }
  /// tile_probabilities == None uses: `DEFAULT_TILE_PROBABILITIES`
  pub fn new_random(size: Size2D, tile_probabilities: Option<TileProbs>) -> Self {
    Self {
      tile_probabilities: tile_probabilities.unwrap_or(DEFAULT_TILE_PROBABILITIES),
      size,
      user_str: UserString::new_random(),
    }
  }
  pub fn user_str(&self) -> &str {
    &self.user_str.0
  }
  pub fn size(&self) -> Size2D {
    self.size
  }
  pub fn tile_probabilities(&self) -> &TileProbs {
    &self.tile_probabilities
  }
}

impl From<&Seed> for String {
  fn from(seed: &Seed) -> Self {
    let Seed {
      size: Size2D { x_size, y_size },
      user_str,
      tile_probabilities,
    } = seed;
    let mut str = format!("{user_str}#{x_size:x}x{y_size:x}");
    if *tile_probabilities != DEFAULT_TILE_PROBABILITIES {
      str.push('#');
      for prob in tile_probabilities {
        write!(str, "{prob:02x}").unwrap();
        //                        Technically this should never unwrap so ignoring the error or unwrap_unchecked should be fine
      }
    }
    str
  }
}
impl From<Seed> for String {
  fn from(seed: Seed) -> Self {
    String::from(&seed)
  }
}

impl TryFrom<&str> for Seed {
  type Error = SeedConversionError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.is_empty() {
      return Err(SeedConversionError::EmptyString);
    }
    let mut parts = value.split('#');
    let user_str_slice = parts.next().unwrap(); // The first split can never fail
    let user_str = UserString::try_from(user_str_slice)?;
    let size = parts
      .next()
      .map(Size2D::try_from)
      .transpose()?
      .unwrap_or(DEFAULT_SIZE);
    let tile_probabilities_slice = parts.next();

    let tile_probabilities = tile_probabilities_slice
      .map(TileProbs::try_from)
      .transpose()?
      .unwrap_or(DEFAULT_TILE_PROBABILITIES);

    if parts.next().is_some() {
      return Err(SeedConversionError::UnexpectedHashTag);
    }
    Ok(Self {
      tile_probabilities,
      size,
      user_str,
    })
  }
}
