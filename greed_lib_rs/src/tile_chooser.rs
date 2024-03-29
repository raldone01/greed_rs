use rand::{distributions::WeightedIndex, prelude::*};

use super::{FakeTile, TileProbs};

pub struct TileChooser<'rng, 'a, RNG: Rng> {
  pub rng: &'rng mut RNG,
  tile_probs: &'a TileProbs,
}

impl<'rng, 'a, RNG: Rng> TileChooser<'rng, 'a, RNG> {
  pub fn new(rng: &'rng mut RNG, tile_probs: &'a TileProbs) -> Self {
    Self { rng, tile_probs }
  }

  pub fn choose(&mut self) -> FakeTile {
    let weights = self.tile_probs.into_iter().map(|&val| u16::from(val));
    let dist = WeightedIndex::new(weights).unwrap();

    #[allow(clippy::cast_possible_truncation)] // dist.sample can only produce values <= 9
    FakeTile::from_unchecked_u8(dist.sample(self.rng) as u8 + 1)
  }
}
