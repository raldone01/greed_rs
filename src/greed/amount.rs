use std::fmt::{Debug, Display};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AmountConverError {
  #[error("Big brother its to big")]
  ToBig,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Amount(u8);

impl Debug for Amount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Debug::fmt(&self.0, f)
  }
}
impl Display for Amount {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self.0, f)
  }
}

impl Amount {
  pub fn amount(self) -> u8 {
    self.0
  }
  pub(super) fn new_unchecked(val: u8) -> Self {
    Self(val)
  }
}

impl From<Amount> for u8 {
  fn from(val: Amount) -> Self {
    val.0
  }
}

impl TryFrom<u8> for Amount {
  type Error = AmountConverError;
  fn try_from(val: u8) -> Result<Self, Self::Error> {
    if val <= 10 {
      Ok(Self(val))
    } else {
      Err(AmountConverError::ToBig)
    }
  }
}

impl TryFrom<usize> for Amount {
  type Error = AmountConverError;
  fn try_from(val: usize) -> Result<Self, Self::Error> {
    if val <= 10 {
      Ok(Self(val as u8)) // safe since val is <= 10
    } else {
      Err(AmountConverError::ToBig)
    }
  }
}
