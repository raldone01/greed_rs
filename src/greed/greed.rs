use bitflags::bitflags;
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use sha2::{Digest, Sha512};

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
    let invalid = (self == (Direction::UP | Direction::DOWN))
      || (self == (Direction::LEFT | Direction::RIGHT))
      || (self.is_empty());
    if invalid {
      Err(GreedError::InvalidDirection)
    } else {
      Ok(())
    }
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
    return &DIRS;
  }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct GameMeta {
  pub file_version: Option<u64>,
  pub greed_version: Option<u64>,
  pub seed: Option<String>,
  pub name: Option<String>,
  pub utc_started_ms: Option<i64>,
  pub utc_finished_ms: Option<i64>,
  pub time_spent_ms: Option<i64>,
  #[serde(default)]
  #[serde_as(as = "Option<Vec<(_, _)>>")]
  pub difficulty_map: Option<DifficultyMap>,
}

type TileAndPos = (Pos, Tile);

#[derive(Clone, Debug, PartialEq)]
pub struct Greed {
  meta: GameMeta,
  field: GameField,
}

impl Greed {
  pub fn new(size: Pos, mut game_meta: GameMeta) -> Self {
    game_meta.greed_version = Some(1);
    game_meta.file_version.get_or_insert(1);

    game_meta
      .utc_started_ms
      .get_or_insert(chrono::Utc::now().timestamp_millis());

    if game_meta.seed.is_none() {
      // If no seed is provided generate one
      let mut thread_rng = thread_rng();
      let uniform = Uniform::new_inclusive('A', 'Z');
      let random_string = (0..512)
        .map(|_| thread_rng.sample(uniform))
        .collect::<String>();
      game_meta.seed = Some(random_string);
    }
    game_meta
      .name
      .get_or_insert(game_meta.seed.clone().unwrap());
    let string_seed = game_meta.seed.as_ref().unwrap();
    let mut hasher = Sha512::new();
    hasher.update(string_seed);
    let hash = hasher.finalize();
    let used_hash = <[u8; 16]>::try_from(&hash[0..16]).unwrap();
    let mut rng = rand_pcg::Pcg64Mcg::from_seed(used_hash); // init the random gen with the first 16 bytes of the hash
    let diff_map = game_meta
      .difficulty_map
      .as_ref()
      .unwrap_or(DifficultyMap::default_difficulties());
    let mut tile_chooser = TileChooser::new(&mut rng, &diff_map);
    let mut game_field = GameField::new_empty(size.x, size.y);
    game_field.randomize_field(&mut tile_chooser);

    Self {
      meta: game_meta,
      field: game_field,
    }
  }
  pub fn game_meta(&self) -> &GameMeta {
    &self.meta
  }
  pub fn field(&self) -> &GameField {
    &self.field
  }
  fn _move(&mut self, dir: Direction, consume: bool) -> Result<Vec<TileAndPos>, GreedError> {
    todo!()
  }
  /// Returns the positions that were consumed.
  /// They are in order from the closest to the farthest.
  pub fn move_(&mut self, dir: Direction) -> Result<Vec<Pos>, GreedError> {
    todo!()
  }
  /// Returns the positions that would be consumed.
  /// They are in order from the closest to the farthest.
  /// # Examples
  /// ```rust
  /// use greed::*;
  ///
  /// let game = Greed::new(...);
  /// let move_score = game.check_move(dir).len();
  /// ```
  pub fn check_move(&mut self, dir: Direction) -> Result<Vec<TileAndPos>, GreedError> {
    todo!()
  }
  pub fn score() -> u128 {
    todo!()
  }
  pub fn time_played() -> std::time::Duration {
    todo!()
  }
}

impl TryFrom<&str> for Greed {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    // Finds the last index of } which must indicate the end of the meta data
    let meta_end_pos = value
      .char_indices()
      .rev()
      .find(|&(_, char)| char == '}')
      // +1 to include } and +1 to include \n
      .map(|(index, _)| index + 2)
      .unwrap_or(0);
    let game_meta = if meta_end_pos != 0 {
      json5::from_str::<GameMeta>(
        value
          .get(0..meta_end_pos)
          .ok_or_else(|| GameFieldParserError::EmptyLine)?,
      )
      .map_err(|cause| GameFieldParserError::InvalidMetaData { cause })?
    } else {
      GameMeta::default()
    };
    let game_field = GameField::try_from(&value[meta_end_pos..])?;
    Ok(Self {
      meta: game_meta,
      field: game_field,
    })
  }
}

impl Into<String> for Greed {
  fn into(self) -> String {
    let out = String::with_capacity(1024 + self.field.tile_count());
    todo!("Save game meta then write game field")
  }
}
