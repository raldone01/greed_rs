use lazy_static::lazy_static;
use rand::{distributions::WeightedIndex, prelude::*};
use std::collections::HashMap;

use super::*;

pub type DifficultyMap = HashMap<Tile, f64>;

pub trait DifficultyMapExt {
  /// TODO remove
  fn saturate_difficulties<'a>(&'a mut self) -> &'a Self;
  /// TODO rename to calculate_percentages
  fn normalize_difficulties<'a>(&'a mut self) -> &'a Self;
  fn default_difficulties() -> &'static Self;
  fn new_difficulty_map() -> Self;
}

impl DifficultyMapExt for DifficultyMap {
  fn saturate_difficulties<'a>(&'a mut self) -> &'a Self {
    let default = DifficultyMap::default_difficulties();
    for tile in default.iter() {
      if self.keys().find(|&ex_key| ex_key == tile.0).is_none() {
        self.insert(*tile.0, *tile.1);
      }
    }
    return self;
  }
  fn normalize_difficulties<'a>(&'a mut self) -> &'a Self {
    let total_probabilities = self.values().fold(0_f64, |accu, prob| accu + prob);
    for value in self.values_mut() {
      *value /= total_probabilities;
    }
    return self;
  }
  /// Equal distribution by default
  fn default_difficulties() -> &'static Self {
    const PROB: f64 = 1_f64 / 8_f64;
    lazy_static! { // sad that rust can evaluate HashMap::from at compile time
      static ref MAP: DifficultyMap = HashMap::from([
      (Tile::Player, 0.0),
      (Tile::EMPTY, 0.0),
      (Tile::V1, PROB),
      (Tile::V2, PROB),
      (Tile::V3, PROB),
      (Tile::V4, PROB),
      (Tile::V5, PROB),
      (Tile::V6, PROB),
      (Tile::V7, PROB),
      (Tile::V8, PROB),
      (Tile::V9, PROB),]);
    }
    return &MAP;
  }
  fn new_difficulty_map() -> Self {
    DifficultyMap::default_difficulties().clone()
  }
}

pub struct TileChooser<'rng, RNG: Rng> {
  pub rng: &'rng mut RNG,
  difficulty_map: Vec<(Tile, f64)>,
}

impl<'rng, RNG: Rng> TileChooser<'rng, RNG> {
  pub fn new(rng: &'rng mut RNG, difficulty_map: &DifficultyMap) -> Self {
    let map = difficulty_map
      .iter()
      .filter_map(|(k, v)| {
        if *k == Tile::Player {
          Some((*k, *v))
        } else {
          None
        }
      })
      .collect::<Vec<_>>(); // stabilize the indexes of the hash map
    Self {
      rng,
      difficulty_map: map,
    }
  }

  /// Never returns a player no matter the probability.
  pub fn choose(&mut self) -> Tile {
    let weights = self.difficulty_map.iter().map(|(_, v)| v);
    let dist = WeightedIndex::new(weights).unwrap();
    return self.difficulty_map[dist.sample(self.rng)].0;
  }
}
