use bitflags::bitflags;
use lazy_static::lazy_static;
use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};
use std::{
  fmt,
  ops::{Mul, Neg},
};

use super::{GreedError, Pos};

bitflags! {
  pub struct Direction: u8 {
    const UP    = 0b0000_0001; // 1
    const DOWN  = 0b0000_0010; // 2
    const LEFT  = 0b0000_0100; // 4
    const RIGHT = 0b0000_1000; // 8
  }
}

impl Direction {
  pub fn is_valid(self) -> bool {
    // Previous Impl:
    //  let invalid = self.reduce().is_empty();
    //  !invalid

    (self.contains(Direction::UP) ^ self.contains(Direction::DOWN))
      | (self.contains(Direction::LEFT) ^ self.contains(Direction::RIGHT))
  }

  pub fn valid(self) -> Result<(), GreedError> {
    if self.is_valid() {
      Ok(())
    } else {
      Err(GreedError::InvalidDirection)
    }
  }

  /// disambiguates a direction
  pub fn reduce(mut self) -> Self {
    if self.contains(Direction::UP | Direction::DOWN) {
      self ^= Direction::UP | Direction::DOWN;
    }
    if self.contains(Direction::LEFT | Direction::RIGHT) {
      self ^= Direction::RIGHT | Direction::LEFT;
    }
    self
  }

  /// Same as !dir
  pub fn reverse(self) -> Self {
    !self
  }

  pub fn all_directions_cw() -> &'static [Direction; 8] {
    lazy_static! { // sad that rust can't evaluate bitflags | at compile time
      static ref DIRS: [Direction; 8] = [
      Direction::UP,
      Direction::UP | Direction::RIGHT,
      Direction::RIGHT,
      Direction::RIGHT | Direction::DOWN,
      Direction::DOWN,
      Direction::DOWN | Direction::LEFT,
      Direction::LEFT,
      Direction::LEFT | Direction::UP,
    ];
    }
    &DIRS
  }
}

impl<T> Mul<T> for Direction
where
  Pos: Mul<T>,
{
  type Output = <Pos as Mul<T>>::Output;

  fn mul(self, rhs: T) -> Self::Output {
    Pos::from(self) * rhs
  }
}

impl Neg for Direction {
  type Output = Direction;

  fn neg(self) -> Self::Output {
    !self
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let dir = self.reduce();
    let str = if dir == Direction::UP {
      "UP"
    } else if dir == Direction::DOWN {
      "DOWN"
    } else if dir == Direction::LEFT {
      "LEFT"
    } else if dir == Direction::RIGHT {
      "RIGHT"
    } else if dir == Direction::UP | Direction::RIGHT {
      "UP_RIGHT"
    } else if dir == Direction::RIGHT | Direction::DOWN {
      "DOWN_RIGHT"
    } else if dir == Direction::DOWN | Direction::LEFT {
      "DOWN_LEFT"
    } else if dir == Direction::LEFT | Direction::UP {
      "UP_LEFT"
    } else {
      "None"
    };
    write!(f, "{}", str)
  }
}

impl Serialize for Direction {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_u8(self.bits)
  }
}

struct DirectionVisitor;

impl<'de> Visitor<'de> for DirectionVisitor {
  type Value = Direction;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("u8 from 0..=15")
  }

  fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    Direction::from_bits(v).ok_or_else(|| E::custom("Invalid direction"))
  }
}

impl<'de> Deserialize<'de> for Direction {
  fn deserialize<D>(deserializer: D) -> Result<Direction, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_u8(DirectionVisitor)
  }
}
