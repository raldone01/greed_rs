use super::*;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FakeTile {
  amount: u8,
}

impl FakeTile {
  pub const EMTPY: FakeTile = FakeTile { amount: 0 };

  pub fn amount(self) -> u8 {
    self.amount
  }
  pub fn from_unchecked(tile: Tile) -> FakeTile {
    Self { amount: tile as u8 }
  }
}

#[derive(Error, Debug, PartialEq)]
#[error("Can't convert player Tile to FakeTile")]
pub struct FakeTileConversionError {}

impl TryFrom<Tile> for FakeTile {
  type Error = FakeTileConversionError;

  fn try_from(value: Tile) -> Result<Self, Self::Error> {
    let amount = value.amount().ok_or(FakeTileConversionError {})?;
    Ok(FakeTile { amount })
  }
}

impl From<FakeTile> for Tile {
  fn from(fake_tile: FakeTile) -> Self {
    Tile::try_from(fake_tile.amount).unwrap()
  }
}

impl Debug for FakeTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", Tile::from(*self))
  }
}
