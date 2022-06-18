use std::collections::*;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
  V0, // Empty space
  V1,
  V2,
  V3,
  V4,
  V5,
  V6,
  V7,
  V8,
  V9,
  At
}

impl Tile {
  pub const Empty: Tile = Tile::V0;

  pub fn choose(difficulty_map: &DifficultyMap) -> Tile { // TODO: move to custom chooser to reuse distribution and rng?
    let map_vec: Vec<_> = difficulty_map.iter().collect();
    let weights = map_vec.iter().map(|(_, v)| *v);
    let dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();
    return *map_vec[dist.sample(& mut rng)].0;
  }
}

struct TileChooser<'rng,RNG> {
  rng: &'rng mut RNG,
  difficulty_map: DifficultyMap,
}

impl<'rng,RNG> TileChooser<'rng,RNG> {
  fn new(rng: &'rng mut RNG, difficulty_map: DifficultyMap) -> Self {
    Self { rng, difficulty_map }
  }
}

struct GameField {
  vec: Vec<Tile>,
  x_size: usize,
  y_size: usize,
}

struct Pos {
  x: usize,
  y: usize
}

type DifficultyMap = HashMap<Tile, f64>;

trait DifficultyMapExt {
  fn saturate_difficulties<'a>(& 'a mut self) -> &'a Self;
  fn normalize_difficulties<'a>(& 'a mut self) -> &'a Self;
  fn default_difficulties() -> &'static Self;
  fn new_difficulty_map() -> Self;
}

impl DifficultyMapExt for DifficultyMap {
  fn saturate_difficulties<'a>(& 'a mut self) -> &'a Self {
    let default = DifficultyMap::default_difficulties();
    for tile in default.iter() {
      if self.keys().find(|&ex_key| ex_key == tile.0).is_none() {
        self.insert(*tile.0, *tile.1);
      }
    }
    return self
  }
  fn normalize_difficulties<'a>(& 'a mut self) -> &'a Self {
    let total_probabilities = self.values().fold(0_f64, |accu, prob| { accu + prob });
    for value in self.values_mut() {
      *value /= total_probabilities;
    }
    return self
  }
  /// Equal distribution by default
  fn default_difficulties() -> &'static Self {
    let prob = 1_f64 / 8_f64;
    let map: &'static DifficultyMap = &HashMap::from([
      (Tile::At, 0.0),
      (Tile::Empty, 0.0),
      (Tile::V1, prob),]);
    return map
  }
  fn new_difficulty_map() -> Self {
    DifficultyMap::default_difficulties().clone()
  }
}

impl GameField {
  pub fn new(rows: u64, cols: u64) -> Self {
    Self { vec: vec![Tile::Empty; usize::try_from(rows * cols).unwrap()], x_size: usize::try_from(cols).unwrap(), y_size: usize::try_from(rows).unwrap() }
  }

  fn set(&mut self, pos: Pos, tile: Tile) {
      self.vec[pos.x + pos.y * self.x_size] = tile
  }

  fn get(&self, pos: Pos) -> Tile {
    self.vec[pos.x + pos.y * self.x_size]
  }

  pub fn randomize_field(& mut self, difficulty_map: & mut DifficultyMap) {
    difficulty_map.saturate_difficulties();
    difficulty_map.normalize_difficulties();
    for tile in self.vec.iter_mut() {
      *tile = Tile::At;
    }
  }

  pub fn locate_player(&self) -> Pos {
    let it = self.vec.iter().position(|tile| *tile == Tile::At).expect("Player not found!");
    let y = it / self.x_size;
    // let x = it % self.x_size;
    let x = self.vec.len() - y * self.x_size;
    return Pos {x, y}
  }
}



struct Greed {
  field: GameField,
  player: Pos,
  email: String,
  sign_in_count: u64,
}