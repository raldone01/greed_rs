use std::{
  fmt,
  ops::{Add, AddAssign, Sub, SubAssign},
};

use super::Direction;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
  pub x: isize,
  pub y: isize,
}

impl From<Pos> for (isize, isize) {
  fn from(pos: Pos) -> Self {
    (pos.x, pos.y)
  }
}

impl From<(isize, isize)> for Pos {
  fn from((x, y): (isize, isize)) -> Self {
    Pos { x, y }
  }
}

impl Add<Direction> for Pos {
  type Output = Pos;

  fn add(self, rhs: Direction) -> Self::Output {
    Pos {
      x: self.x + isize::from(rhs.contains(Direction::RIGHT))
        - isize::from(rhs.contains(Direction::LEFT)),
      y: self.y + isize::from(rhs.contains(Direction::DOWN))
        - isize::from(rhs.contains(Direction::UP)),
    }
  }
}

impl AddAssign<Direction> for Pos {
  fn add_assign(&mut self, rhs: Direction) {
    self.x += isize::from(rhs.contains(Direction::RIGHT));
    self.x -= isize::from(rhs.contains(Direction::LEFT));
    self.y += isize::from(rhs.contains(Direction::DOWN));
    self.y -= isize::from(rhs.contains(Direction::UP));
  }
}

impl Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl AddAssign for Pos {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl Sub for Pos {
  type Output = Pos;

  fn sub(self, rhs: Self) -> Self::Output {
    Pos {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl SubAssign for Pos {
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
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
