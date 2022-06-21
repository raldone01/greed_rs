use bitflags::bitflags;
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha512};
use std::collections::*;
use std::fmt;
use std::os::windows::thread;

use super::*;

struct TileChooser<'rng, RNG: Rng> {
  rng: &'rng mut RNG,
  difficulty_map: Vec<(Tile, f64)>,
}

impl<'rng, RNG> TileChooser<'rng, RNG> {
  fn new(rng: &'rng mut RNG, difficulty_map: &DifficultyMap) -> Self {
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

  fn choose(&self) -> Tile {
    let weights = self.difficulty_map.iter().map(|(_, v)| v);
    let dist = WeightedIndex::new(weights).unwrap();
    return self.difficulty_map[dist.sample(self.rng)].0;
  }
}

pub type DifficultyMap = HashMap<Tile, f64>;

pub trait DifficultyMapExt {
  pub fn saturate_difficulties<'a>(&'a mut self) -> &'a Self;
  pub fn normalize_difficulties<'a>(&'a mut self) -> &'a Self;
  pub fn default_difficulties() -> &'static Self;
  pub fn new_difficulty_map() -> Self;
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
      (Tile::Empty, 0.0),
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

pub struct GameField {
  vec: Vec<Tile>,
  x_size: usize,
  y_size: usize,
}

pub struct RowIter<'a> {
  game_field: &'a GameField,
  row: usize,
}

impl<'a> Iterator for RowIter<'a> {
  type Item = &'a [Tile];

  fn next(&mut self) -> Option<Self::Item> {
    let &GameField { x_size, y_size, .. } = self.game_field;
    if self.row < y_size {
      let offset = self.row * y_size;
      self.row += 1;
      Ok(&self.game_field.vec[offset..offset + x_size])
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.game_field.y_size, Some(self.game_field.y_size))
  }
}

impl<'a> DoubleEndedIterator for RowIter<'a> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let &GameField { x_size, y_size, .. } = self.game_field;
    if self.row > 0 {
      self.row -= 1;
      let offset = self.row * y_size;
      Ok(&self.game_field.vec[offset..offset + x_size])
    } else {
      None
    }
  }
}

impl<'a> ExactSizeIterator for RowIter<'a> {}

impl GameField {
  pub fn new(rows: u64, cols: u64) -> Self {
    // TODO SEED HERE AND RANDOMIZE
    Self {
      vec: vec![Tile::Empty; usize::try_from(rows * cols).unwrap()],
      x_size: usize::try_from(cols).unwrap(),
      y_size: usize::try_from(rows).unwrap(),
    }
  }

  fn pos_to_index(&self, pos: Pos) -> usize {
    pos.x + pos.y * self.x_size
  }

  fn index_to_pos(&self, index: usize) -> Pos {
    let y = it / self.x_size;
    let x = it % self.x_size;
    // let x = self.vec.len() - y * self.x_size;
    Pos { x, y }
  }

  fn rows(&self) -> RowIter {
    RowIter {
      game_field: self,
      row: 0,
    }
  }

  fn index_mut(&mut self, pos: Pos) -> &mut Tile {
    &mut self.vec[self.pos_to_index(pos)]
  }

  pub fn index(&self, pos: Pos) -> Tile {
    self.vec[pos.x + pos.y * self.x_size]
  }

  fn randomize_field(&mut self, tile_chooser: &TileChooser<impl Rng>) {
    for tile in self.vec.iter_mut() {
      *tile = tile_chooser.choose();
    }
    let pp = tile_chooser.rng.gen_range(0..self.vec.len());
    self.vec[pp] = Tile::Player;
  }

  /// Assumes that EXACTLY one player exists on the game field.
  pub fn locate_player(&self) -> Pos {
    let it = self
      .vec
      .iter()
      .position(|tile| *tile == Tile::Player)
      .expect("Player not found!");
    self.index_to_pos(it)
  }
}

impl From<GameField> for String {
  fn from(game_field: GameField) -> Self {
    let out = String::with_capacity(game_field.x_size * game_field.y_size + game_field.y_size);
    for row in game_field.rows() {
      for tile in row {
        out.push(tile.into())
      }
      out.push('\n')
    }
    out
  }
}

impl TryFrom<&str> for GameField {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut vec = Vec::with_capacity(64 * 64); // TODO: check default games size

    let mut x_size = None;
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut player_seen = false;
    for c in value.chars() {
      match c {
        '\n' => {
          if x_pos == 0 {
            return Err(GameFieldParserError::EmptyLine);
          }
          if let Some(x_size) = x_size {
            if x_pos != x_size {
              return Err(GameFieldParserError::NotRectangular);
            }
          } else {
            x_size = Some(x_pos);
          }
          x_pos = 0;
          y_pos += 1;
        },
        c => {
          let tile = Tile::try_from(c).map_err(|err| {
            Err(GameFieldParserError::InvalidCharacter {
              found: err.found,
              pos: Pos { x: x_pos, y: y_pos },
            })
          })?;
          if tile == Tile::Player {
            if player_seen {
              return Err(GameFieldParserError::AmbiguousPlayer);
            }
            player_seen = true
          }
          vec.push(tile)
        },
      }
    }

    if !player_seen {
      return Err(GameFieldParserError::PlayerNotFound);
    }

    let game_field = GameField {
      vec,
      x_size: x_size.unwrap_or(1),
      y_size: y_pos,
    };
    Ok(game_field)
  }
}

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
    let invalid = (self == (Direction::UP | Direction::DOWN))
      || (self == (Direction::LEFT | Direction::RIGHT))
      || (self.is_empty());
    if invalid {
      Err(GreedError::InvalidDirection)
    } else {
      Ok(())
    }
  }

  pub const fn all_directions_cw() -> &'static [&Direction] {
    static DIRS: [&Direction] = [
      Direction::UP,
      Direction::UP | Direction::RIGHT,
      Direction::RIGHT,
      Direction::RIGHT | Direction::DOWN,
      Direction::DOWN,
      Direction::DOWN | Direction::LEFT,
      Direction::LEFT,
      Direction::LEFT | Direction::UP,
    ];
    return &DIRS;
  }
}

#[derive(Clone, Default)]
pub struct GameMeta {
  pub version: Option<u64>,
  pub seed: Option<String>,
  pub name: String,
  pub utc_started_ms: Option<u64>,
  pub difficulty_map: Option<DifficultyMap>,
}

#[derive(Clone)]
pub struct Greed {
  meta: GameMeta,
  field: GameField,
}

impl Greed {
  pub fn new(size: Pos, mut game_meta: GameMeta) -> Self {
    if (game_meta.seed.is_none()) {
      let mut thread_rng = thread_rng();
      let uniform = Uniform::new_inclusive('A', 'Z');
      let random_string = (0..512)
        .map(|_| thread_rng.sample(uniform))
        .collect::<String>();
      game_meta.seed = Some(random_string);
    }
    let string_seed = &game_meta.seed.unwrap();
    let mut hasher = Sha512::new();
    hasher.update(string_seed);
    let result = hasher.finalize();

    let rng = rand_pcg::Pcg64Mcg::from_seed(result);
    let tile_chooser = TileChooser();
    Self {
      meta: game_meta,
      field: GameField {
        vec: (),
        x_size: (),
        y_size: (),
      },
    }
  }
  pub fn game_meta(&self) -> &GameMeta {
    &self.meta
  }
  pub fn field(&self) -> &GameField {
    &self.field
  }
  fn _move(&mut self, dir: Direction, consume: bool) -> Result<[Pos], GreedError> {}
  /// Returns the positions that were consumed.
  /// They are in order from the closest to the furtherest.
  pub fn move_(&mut self, dir: Direction) -> Result<[Pos], GreedError> {}
  /// Returns the positions that would be consumed.
  /// They are in order from the closest to the furtherest.
  /// # Examples
  /// ```
  /// use greed::*;
  ///
  /// let game = Greed::new(...);
  /// let move_score = game.check_move(dir).len();
  /// ```
  pub fn check_move(&mut self, dir: Direction) -> Result<[Pos], GreedError> {}
  pub fn score() -> u128 {}
  pub fn time_played() -> std::time::Duration {}
}

impl TryFrom<String> for Greed {
  type Error = GameFieldParserError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    todo!("Load game meta")
  }
}

impl Into<String> for Greed {
  fn into(self) -> String {
    todo!("Save game meta")
  }
}
