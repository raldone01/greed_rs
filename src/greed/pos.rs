use serde::{Deserialize, Serialize};
use std::{
  fmt,
  ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use super::{Amount, Direction};

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pos {
  pub x: isize,
  pub y: isize,
}

impl Pos {
  pub fn new(x: isize, y: isize) -> Self {
    Pos { x, y }
  }
}

impl From<Pos> for (isize, isize) {
  fn from(pos: Pos) -> Self {
    (pos.x, pos.y)
  }
}

impl From<(isize, isize)> for Pos {
  fn from((x, y): (isize, isize)) -> Self {
    Pos::new(x, y)
  }
}

impl From<Direction> for Pos {
  fn from(dir: Direction) -> Self {
    Pos::new(
      isize::from(dir.contains(Direction::RIGHT)) - isize::from(dir.contains(Direction::LEFT)),
      isize::from(dir.contains(Direction::DOWN)) - isize::from(dir.contains(Direction::UP)),
    )
  }
}

impl Add<Direction> for Pos {
  type Output = Pos;

  fn add(self, rhs: Direction) -> Self::Output {
    self + Pos::from(rhs)
  }
}

impl AddAssign<Direction> for Pos {
  fn add_assign(&mut self, rhs: Direction) {
    *self += Pos::from(rhs)
  }
}

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl AddAssign for Pos {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl Neg for Pos {
  type Output = Pos;

  fn neg(self) -> Self::Output {
    Pos::new(-self.x, -self.y)
  }
}

impl Sub for Pos {
  type Output = Pos;

  #[allow(clippy::suspicious_arithmetic_impl)] // Clippy is a bit stupid
  fn sub(self, rhs: Self) -> Self::Output {
    self + rhs.neg()
    // self + -rhs
  }
}

impl SubAssign for Pos {
  fn sub_assign(&mut self, rhs: Self) {
    *self += -rhs;
  }
}

impl<T: Into<isize>> Mul<T> for Pos {
  type Output = Pos;

  fn mul(self, rhs: T) -> Self::Output {
    let mult = rhs.into();
    Pos {
      x: self.x * mult,
      y: self.y * mult,
    }
  }
}

impl Mul<Amount> for Pos {
  type Output = Pos;

  fn mul(self, rhs: Amount) -> Self::Output {
    Pos {
      x: self.x * isize::from(rhs.amount()),
      y: self.y * isize::from(rhs.amount()),
    }
  }
}

impl<T: Into<isize> + Clone> MulAssign<T> for Pos {
  fn mul_assign(&mut self, rhs: T) {
    let mult = rhs.into();
    self.x *= mult;
    self.y *= mult;
  }
}

impl MulAssign<Amount> for Pos {
  fn mul_assign(&mut self, rhs: Amount) {
    self.x *= isize::from(rhs.amount());
    self.y *= isize::from(rhs.amount());
  }
}

impl fmt::Display for Pos {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

impl fmt::Debug for Pos {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Pos{}", self)
  }
}
