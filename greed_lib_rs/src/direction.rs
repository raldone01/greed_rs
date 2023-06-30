use super::Pos;
use arbitrary::Arbitrary;
use bitflags::bitflags;
use core::{
  fmt,
  ops::{Mul, Neg},
};
use serde::{de, Deserialize, Deserializer, Serialize};

bitflags! {
  pub struct Direction: u8 {
    const UP    = 0b0000_0001; // 1
    const DOWN  = 0b0000_0010; // 2
    const LEFT  = 0b0000_0100; // 4
    const RIGHT = 0b0000_1000; // 8
  }
}

impl Direction {
  pub const ALL_DIRECTIONS_CW: [Self; 8] = [
    Self::UP,
    Self::UP.union(Self::RIGHT),
    Self::RIGHT,
    Self::RIGHT.union(Self::DOWN),
    Self::DOWN,
    Self::DOWN.union(Self::LEFT),
    Self::LEFT,
    Self::LEFT.union(Self::UP),
  ];
  /// Checks if a direction is valid.
  /// A valid direction must actually change position when moved.
  /// thereby `0`, `UP | DOWN`, `LEFT | RIGHT` and  `LEFT | RIGHT | UP | DOWN` are invalid
  #[must_use]
  pub const fn is_valid(self) -> bool {
    // Previous Impl:
    //  let invalid = self.reduce().is_empty();
    //  !invalid

    (self.contains(Self::UP) ^ self.contains(Self::DOWN))
      | (self.contains(Self::LEFT) ^ self.contains(Self::RIGHT))
  }

  /// disambiguates a direction
  #[must_use]
  pub fn reduce(mut self) -> Self {
    if self.contains(Self::UP | Self::DOWN) {
      self ^= Self::UP | Self::DOWN;
    }
    if self.contains(Self::LEFT | Self::RIGHT) {
      self ^= Self::RIGHT | Self::LEFT;
    }
    self
  }

  /// Same as !dir
  #[must_use]
  pub fn reverse(self) -> Self {
    !self
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
  type Output = Self;

  fn neg(self) -> Self::Output {
    !self
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let dir = self.reduce();
    let str = if dir == Self::UP {
      "UP"
    } else if dir == Self::DOWN {
      "DOWN"
    } else if dir == Self::LEFT {
      "LEFT"
    } else if dir == Self::RIGHT {
      "RIGHT"
    } else if dir == Self::UP | Self::RIGHT {
      "UP_RIGHT"
    } else if dir == Self::RIGHT | Self::DOWN {
      "DOWN_RIGHT"
    } else if dir == Self::DOWN | Self::LEFT {
      "DOWN_LEFT"
    } else if dir == Self::LEFT | Self::UP {
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

impl<'de> Deserialize<'de> for Direction {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Self::from_bits(u8::deserialize(deserializer)?)
      .ok_or_else(|| de::Error::custom("Invalid direction"))
  }
}

impl<'a> Arbitrary<'a> for Direction {
  fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
    #[allow(clippy::cast_possible_truncation)] // Can never truncate since it is always < 16
    unsafe {
      Ok(Self::from_bits_unchecked(u.choose_index(16)? as u8))
    }
  }
}
