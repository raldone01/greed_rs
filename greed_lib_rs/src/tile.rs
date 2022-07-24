use num_enum::TryFromPrimitive;
use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};
use std::fmt;

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
    Tile::try_from(v).map_err(|err| E::custom(err.to_string()))
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
    match tile {
      Tile::Player => '@',
      // V0 => ' ', Handled in the display function
      // x if let Some(amount) = self.amount() => { Ok(()) } Unstable damn
      _ => tile
        .amount()
        .map(|a| char::from_digit(u32::from(a), 10).unwrap())
        .unwrap(),
    }
  }
}

impl TryFrom<char> for Tile {
  type Error = TileParseError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '@' => Ok(Tile::Player),
      ' ' => Ok(Tile::V0),
      #[allow(clippy::cast_possible_truncation)] // Always is <= 9
      c => c
        .to_digit(10)
        .map(|num| Tile::try_from(num as u8).unwrap())
        .ok_or(TileParseError { found: c }),
    }
  }
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut c = char::from(*self);
    if c == '0' {
      c = ' ';
    }
    write!(f, "{}", c)
  }
}

impl fmt::Debug for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Tile{self}")
  }
}
