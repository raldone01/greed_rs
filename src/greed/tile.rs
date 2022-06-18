use num_enum::TryFromPrimitive;
use std::fmt;
use thiserror::Error;

use super::*;

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

impl Tile {
  pub const EMPTY: Tile = Tile::V0;

  pub fn amount(self) -> Option<u8> {
    if self == Tile::Player {
      None
    } else {
      Some(self as u8)
    }
  }

  pub fn is_player(self) -> bool {
    self == Tile::Player
  }

  pub fn is_empty(self) -> bool {
    self == Tile::Empty
  }
}

impl From<Tile> for char {
  fn from(tile: Tile) -> Self {
    match tile {
      Player => '@',
      V0 => ' ',
      // x if let Some(amount) = self.amount() => { Ok(()) } Unstable damn
      _ => tile.amount().map(|a| char::from_digit(a, 10)).unwrap(),
    }
  }
}

impl TryFrom<char> for Tile {
  type Error = TileParseError;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '@' => Ok(Tile::Player),
      ' ' => Ok(Tile::V0),
      c => c
        .to_digit(10)
        .map(|num| Tile::try_from(num as u8).unwrap())
        .ok_or(TileParseError { found: c }),
    }
  }
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.into())
  }
}

impl fmt::Debug for Tile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Tile{self}")
  }
}
