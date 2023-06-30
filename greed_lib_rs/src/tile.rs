use alloc::format;
use core::fmt::{self, Debug, Display, Formatter};
use num_enum::TryFromPrimitive;
use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};

use super::TileParseError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum Tile {
  V0 = 0, // Empty space
  V1,
  V2,
  V3,
  V4,
  V5,
  V6,
  V7,
  V8,
  V9,
  Player,
}

impl Serialize for Tile {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_char(char::from(*self))
  }
}

struct TileVisitor;

impl<'de> Visitor<'de> for TileVisitor {
  type Value = Tile;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("one of 0123456789 @")
  }

  fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Tile::try_from(v).map_err(|err| E::custom(format!("{}", err)))
  }
}

impl<'de> Deserialize<'de> for Tile {
  fn deserialize<D>(deserializer: D) -> Result<Tile, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_char(TileVisitor)
  }
}

impl Tile {
  pub const EMPTY: Tile = Tile::V0;

  #[must_use]
  pub fn amount(self) -> Option<u8> {
    if self == Tile::Player {
      None
    } else {
      Some(self as u8)
    }
  }

  #[must_use]
  pub fn is_player(self) -> bool {
    self == Tile::Player
  }

  #[must_use]
  pub fn is_empty(self) -> bool {
    self == Tile::EMPTY
  }
}

impl From<Tile> for char {
  fn from(tile: Tile) -> Self {
    if let Some(amount) = tile.amount() {
      Self::from_digit(u32::from(amount), 10).unwrap()
    } else {
      debug_assert_eq!(tile, Tile::Player);
      // since .amount() failed we must have a player tile, debug assert to make sure added tile types don't silently do something wrong
      '@'
    }
  }
}

impl TryFrom<char> for Tile {
  type Error = TileParseError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '@' => Ok(Self::Player),
      ' ' => Ok(Self::V0),
      #[allow(clippy::cast_possible_truncation)] // Always is <= 9
      c => c
        .to_digit(10)
        .map(|num| Self::try_from(num as u8).unwrap())
        .ok_or(TileParseError { found: c }),
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut c = char::from(*self);
    if c == '0' {
      c = ' ';
    }
    write!(f, "{}", c)
  }
}

impl Debug for Tile {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Tile{self}")
  }
}
