use super::{amount::AmountConversionError, Amount, Tile};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
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
pub enum FakeTileConversionError {
  #[error("Can't convert player Tile to FakeTile")]
  PlayerTile,
}

impl From<FakeTile> for u8 {
  fn from(value: FakeTile) -> Self {
    value.amount
  }
}

impl TryFrom<u8> for FakeTile {
  type Error = AmountConversionError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(Self {
      amount: Amount::try_from(value)?.amount(),
    })
  }
}

impl TryFrom<Tile> for FakeTile {
  type Error = FakeTileConversionError;

  fn try_from(value: Tile) -> Result<Self, Self::Error> {
    let amount = value.amount().ok_or(FakeTileConversionError::PlayerTile)?;
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
