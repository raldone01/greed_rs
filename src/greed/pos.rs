use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
  pub x: usize,
  pub y: usize,
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
