use core::fmt::{self, Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AmountConversionError {
  #[error("Big brother its to big")]
  ToBig,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub struct Amount(u8);

impl Debug for Amount {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Debug::fmt(&self.0, f)
  }
}
impl Display for Amount {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    Display::fmt(&self.0, f)
  }
}

impl Amount {
  #[must_use]
  pub const fn amount(self) -> u8 {
    self.0
  }
  #[must_use]
  pub(super) const fn new_unchecked(val: u8) -> Self {
    Self(val)
  }
}

impl From<Amount> for u8 {
  fn from(val: Amount) -> Self {
    val.0
  }
}

impl TryFrom<u8> for Amount {
  type Error = AmountConversionError;
  fn try_from(val: u8) -> Result<Self, Self::Error> {
    if val < 10 {
      Ok(Self(val))
    } else {
      Err(AmountConversionError::ToBig)
    }
  }
}

impl TryFrom<usize> for Amount {
  type Error = AmountConversionError;
  fn try_from(val: usize) -> Result<Self, Self::Error> {
    if val < 10 {
      #[allow(clippy::cast_possible_truncation)] // Never actually truncates since `val < 10`
      Ok(Self(val as u8)) // safe since val is < 10
    } else {
      Err(AmountConversionError::ToBig)
    }
  }
}
