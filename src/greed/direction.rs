use bitflags::bitflags;
use lazy_static::lazy_static;
use serde::{
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};
use std::fmt;

use super::*;

bitflags! {
  pub struct Direction: u8 {
    const UP    = 0b00000001;
    const DOWN  = 0b00000010;
    const LEFT  = 0b00000100;
    const RIGHT = 0b00001000;
  }
}

impl Direction {
  pub fn valid(self) -> Result<(), GreedError> {
    let invalid = self.reduce().is_empty();
    if invalid {
      Err(GreedError::InvalidDirection)
    } else {
      Ok(())
    }
  }

  // disambiguates a direction
  pub fn reduce(mut self) -> Self {
    if self.contains(Direction::UP | Direction::DOWN) {
      self ^= Direction::UP | Direction::DOWN;
    }
    if self.contains(Direction::LEFT | Direction::RIGHT) {
      self ^= Direction::RIGHT | Direction::LEFT;
    }
    self
  }

  // Same as !dir
  pub fn reverse(self) -> Self {
    !self
  }

  pub fn all_directions_cw() -> &'static [Direction; 8] {
    lazy_static! { // sad that rust can evaluate bitflags | at compile time
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
    Direction::from_bits(v).ok_or(E::custom("Invalid direction"))
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
