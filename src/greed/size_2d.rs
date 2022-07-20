use core::fmt;
use serde::{Deserialize, Serialize};
use std::num::TryFromIntError;
use thiserror::Error;

use super::Pos;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "(usize, usize)")]
#[serde(into = "(usize, usize)")]
pub struct Size2D {
  pub x_size: usize,
  pub y_size: usize,
}

impl Size2D {
  pub(super) fn new_unchecked(x_size: usize, y_size: usize) -> Self {
    Self { x_size, y_size }
  }
  pub fn new(x_size: usize, y_size: usize) -> Result<Self, Size2DConversionError> {
    if x_size == 0 || y_size == 0 || x_size > isize::MAX as usize || y_size > isize::MAX as usize {
      return Err(Size2DConversionError::OutOfRange);
    }
    Ok(Self::new_unchecked(x_size, y_size))
  }
  pub fn tile_count(&self) -> usize {
    self.x_size * self.y_size
  }
}

impl TryFrom<Size2D> for Pos {
  type Error = TryFromIntError;

  fn try_from(value: Size2D) -> Result<Self, Self::Error> {
    Ok(Pos::new(
      isize::try_from(value.x_size)?,
      isize::try_from(value.y_size)?,
    ))
  }
}
impl TryFrom<Pos> for Size2D {
  type Error = Size2DConversionError;

  fn try_from(value: Pos) -> Result<Self, Self::Error> {
    Size2D::new(
      usize::try_from(value.x).map_err(|_| Size2DConversionError::OutOfRange)?,
      usize::try_from(value.y).map_err(|_| Size2DConversionError::OutOfRange)?,
    )
  }
}

impl From<Size2D> for (usize, usize) {
  fn from(value: Size2D) -> Self {
    (value.x_size, value.y_size)
  }
}

impl TryFrom<(usize, usize)> for Size2D {
  type Error = Size2DConversionError;

  fn try_from((x_size, y_size): (usize, usize)) -> Result<Self, Self::Error> {
    Self::new(x_size, y_size)
  }
}

#[derive(Error, Debug, PartialEq)]
pub enum Size2DConversionError {
  #[error("Invalid Dimensions Format")] // Maybe split into multiple Errors
  InvalidFormat,
  #[error("Dimension out of range (x_size: 1..={0}, y_size: 1..={0}", isize::MAX)]
  OutOfRange,
}

impl TryFrom<&str> for Size2D {
  type Error = Size2DConversionError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut splits = value.split('x');
    let x_size = splits.next().unwrap(); // The first split can never be None
    let y_size = splits.next().ok_or(Size2DConversionError::InvalidFormat)?;
    if splits.next().is_some() {
      return Err(Size2DConversionError::InvalidFormat);
    }
    let x_size =
      usize::from_str_radix(x_size, 16).map_err(|_| Size2DConversionError::InvalidFormat)?;
    let y_size =
      usize::from_str_radix(y_size, 16).map_err(|_| Size2DConversionError::InvalidFormat)?;
    Self::new(x_size, y_size)
  }
}

impl fmt::Display for Size2D {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.x_size, self.y_size)
  }
}
