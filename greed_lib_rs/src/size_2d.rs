use super::{Grid2D, Pos};
use arbitrary::Arbitrary;
use core::{fmt, num::TryFromIntError, ops::RangeInclusive};
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(try_from = "(usize, usize)")]
#[serde(into = "(usize, usize)")]
pub struct Size2D {
  pub x_size: usize,
  pub y_size: usize,
}

pub const DEFAULT_SIZE: Size2D = Size2D {
  x_size: 79,
  y_size: 21,
};

#[cfg(not(fuzz))]
pub const MAX_TILE_COUNT: usize = 65536; // Max 64kB (for now) (256 x 265 Grid)
                                         // Must be smaller than isize::MAX

#[cfg(fuzz)]
pub const MAX_TILE_COUNT: usize = 1024; // Limit the size to 32x32 for fuzzing to speed it up

impl Size2D {
  pub(super) fn new_unchecked(x_size: usize, y_size: usize) -> Self {
    Self { x_size, y_size }
  }
  /// Creates a now `Size2D`, checking if it is actually valid.
  /// # Errors
  /// * If any Dimension is zero.
  /// * The product of the Dimensions overflows.
  /// * The product of the Dimensions is larger than `MAX_TILE_COUNT`.
  pub fn new(x_size: usize, y_size: usize) -> Result<Self, Size2DConversionError> {
    let (tile_count, overflow) = x_size.overflowing_mul(y_size);
    if tile_count == 0 {
      return Err(Size2DConversionError::ZeroSize);
    }
    if overflow || tile_count > MAX_TILE_COUNT {
      return Err(Size2DConversionError::SizeOutOfRange {
        actual_size: if overflow { usize::MAX } else { tile_count },
      });
    }
    Ok(Self::new_unchecked(x_size, y_size))
  }
  #[must_use]
  pub fn tile_count(&self) -> usize {
    self.x_size * self.y_size
  }
}

impl Grid2D for Size2D {
  fn dimensions(&self) -> Size2D {
    *self
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
      usize::try_from(value.x).map_err(|_| Size2DConversionError::Negative)?,
      usize::try_from(value.y).map_err(|_| Size2DConversionError::Negative)?,
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
  #[error("The size {actual_size} is bigger then the defined limit {MAX_TILE_COUNT}")]
  SizeOutOfRange { actual_size: usize },
  #[error("Size with tile_count zero are not allowed")]
  ZeroSize,
  #[error("Negative Dimensions are not allowed")]
  Negative,
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

impl<'a> Arbitrary<'a> for Size2D {
  fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
    // Limit the size to "reasonable" values
    const RANGE: RangeInclusive<usize> = 1..=80;
    Self::new(u.int_in_range(RANGE)?, u.int_in_range(RANGE)?)
      .map_err(|_| arbitrary::Error::IncorrectFormat)
  }
}
