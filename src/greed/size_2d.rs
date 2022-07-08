use std::num::TryFromIntError;

use super::*;

#[non_exhaustive]
pub struct Size2D {
  pub x_size: usize,
  pub y_size: usize,
}

impl Size2D {
  pub fn new(x_size: usize, y_size: usize) -> Self {
    Self { x_size, y_size }
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
  type Error = TryFromIntError;

  fn try_from(value: Pos) -> Result<Self, Self::Error> {
    Ok(Size2D::new(
      usize::try_from(value.x)?,
      usize::try_from(value.y)?,
    ))
  }
}

impl From<Size2D> for (usize, usize) {
  fn from(value: Size2D) -> Self {
    (value.x_size, value.y_size)
  }
}

impl From<(usize, usize)> for Size2D {
  fn from((x_size, y_size): (usize, usize)) -> Self {
    Size2D::new(x_size, y_size)
  }
}
