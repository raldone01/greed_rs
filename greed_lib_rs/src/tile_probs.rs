use arbitrary::Arbitrary;
use thiserror_no_std::Error;

type Inner = [u8; 9];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileProbs(Inner);

pub const DEFAULT_TILE_PROBABILITIES: TileProbs = TileProbs([1, 1, 1, 1, 1, 1, 1, 1, 1]);

impl IntoIterator for TileProbs {
  type Item = <Inner as IntoIterator>::Item;
  type IntoIter = <Inner as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}
impl<'a> IntoIterator for &'a TileProbs {
  type Item = <&'a Inner as IntoIterator>::Item;
  type IntoIter = <&'a Inner as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum TileProbsConversionError {
  #[error("Empty Tile Probabilities Field")]
  Empty,
  #[error("Invalid Tile Probabilities Format")]
  InvalidFormat,
  #[error("All Probabilities are Zero")]
  AllZeros,
  #[error("Invalid Char in Tile Probabilities Format: {c}")]
  InvalidChar { c: char },
  #[error("The wrong amount of chars was given ({count} given, expected 18")]
  InvalidCharCount { count: usize },
}

impl TryFrom<&str> for TileProbs {
  type Error = TileProbsConversionError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(c) = value.chars().find(|c| !c.is_ascii_hexdigit()) {
      return Err(TileProbsConversionError::InvalidChar { c });
    }
    if value.len() != 2 * 9 {
      return Err(TileProbsConversionError::InvalidCharCount { count: value.len() });
    }
    let mut val_slices = value.as_bytes().chunks(2);
    let mut vals = [0; 9];
    for val in &mut vals {
      let val_slice = val_slices.next().unwrap(); // since we checked the size previously this check is redundant
      let val_slice =
        core::str::from_utf8(val_slice).map_err(|_| TileProbsConversionError::InvalidFormat)?;
      *val =
        u8::from_str_radix(val_slice, 16).map_err(|_| TileProbsConversionError::InvalidFormat)?;
    }
    TileProbs::try_from(vals)
  }
}
impl From<TileProbs> for Inner {
  fn from(props: TileProbs) -> Self {
    props.0
  }
}

impl TryFrom<Inner> for TileProbs {
  type Error = TileProbsConversionError;

  fn try_from(value: Inner) -> Result<Self, Self::Error> {
    if value == [0; 9] {
      return Err(TileProbsConversionError::AllZeros);
    }
    Ok(Self(value))
  }
}

impl<'a> Arbitrary<'a> for TileProbs {
  fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
    Self::try_from(<[u8; 9]>::arbitrary(u)?).map_err(|_| arbitrary::Error::IncorrectFormat)
  }
}
