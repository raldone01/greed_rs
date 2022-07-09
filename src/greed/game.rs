use super::*;
use chrono::{DateTime, Utc};
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use sha2::{Digest, Sha512};

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
  pub moves: Option<Vec<Direction>>,
  pub score: Option<usize>,
  /// A score based on time spent, moves (counting undos) and a few more factors.
  /// This value can't be verified and is designed to increase human engagement.
  /// TODO: Maybe also store average_move_time.
  pub human_score: Option<usize>,
  pub undos: Option<usize>,
}

pub struct GreedBuilder {
  pub seed: Option<String>,
  pub name: Option<String>,
  pub difficulty_map: Option<DifficultyMap>,
  size: Size2D,
}

impl GreedBuilder {
  pub fn new() -> Self {
    Self {
      seed: None,
      name: None,
      difficulty_map: None,
      size: GameField::default_classic_game_dimensions(),
    }
  }

  pub fn resize(&mut self, size: Size2D) -> Result<&mut Self, GameFieldParserError> {
    let Size2D { x_size, y_size, .. } = size;
    if x_size < 1 || y_size < 1 || x_size > isize::MAX as usize || y_size > isize::MAX as usize {
      return Err(GameFieldParserError::InvalidSize);
    }
    self.size = size;
    Ok(self)
  }

  fn gen_rand_seed_str() -> String {
    let mut thread_rng = thread_rng();
    let uniform = Uniform::new_inclusive('A', 'Z');
    let random_string = (0..512)
      .map(|_| thread_rng.sample(uniform))
      .collect::<String>();
    random_string
  }

  pub fn rand_seed(&mut self) -> &mut Self {
    self.seed = Some(GreedBuilder::gen_rand_seed_str());
    self
  }

  pub fn seed(&mut self, seed: &str) -> &mut Self {
    self.seed = Some(String::from(seed));
    self
  }

  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = Some(String::from(name));
    self
  }

  pub fn difficulty_map(&mut self, difficulty_map: DifficultyMap) {
    self.difficulty_map = Some(difficulty_map.clone());
  }

  pub fn build(&self) -> Greed {
    let seed = self
      .seed
      .clone()
      .unwrap_or(GreedBuilder::gen_rand_seed_str());

    let name = self.name.clone().unwrap_or(seed);

    let difficulty_map = self
      .difficulty_map
      .clone()
      .unwrap_or(DifficultyMap::default_difficulties().clone());

    Greed::new_from_builder(size, seed, name, difficulty_map)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Greed {
  seed: String,
  name: String,
  utc_started_ms: DateTime<Utc>,
  utc_finished_ms: Option<DateTime<Utc>>,
  difficulty_map: DifficultyMap,
  undos: usize,
  game_state: GameState,
}

impl Greed {
  fn new_from_builder(
    size: Size2D,
    string_seed: String,
    name: String,
    difficulty_map: DifficultyMap,
  ) -> Self {
    let mut hasher = Sha512::new();
    hasher.update(string_seed);
    let hash = hasher.finalize();
    let used_hash = <[u8; 16]>::try_from(&hash[0..16]).unwrap();
    // init the random gen with the first 16 bytes of the hash
    let mut rng = rand_pcg::Pcg64Mcg::from_seed(used_hash);
    #[allow(clippy::or_fun_call)] // TODO: Create an issue
    let diff_map = game_meta
      .difficulty_map
      .as_ref()
      .unwrap_or(DifficultyMap::default_difficulties());
    let mut tile_chooser = TileChooser::new(&mut rng, diff_map);
    let mut game_field = GameField::new_empty(x_size, y_size);
    game_field.randomize_field(&mut tile_chooser);

    Greed {
      seed: string_seed,
      name,
      utc_started_ms: chrono::Utc::now(),
      utc_finished_ms: None,
      difficulty_map,
      undos: 0,
      game_state: (),
    }
  }

  pub fn new(x_size: usize, y_size: usize, mut game_meta: GameMeta) -> Self {
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
    #[allow(clippy::or_fun_call)] // TODO: Create an issue
    let diff_map = game_meta
      .difficulty_map
      .as_ref()
      .unwrap_or(DifficultyMap::default_difficulties());
    let mut tile_chooser = TileChooser::new(&mut rng, diff_map);
    let mut game_field = GameField::new_empty(x_size, y_size);
    game_field.randomize_field(&mut tile_chooser);

    todo!();
    /* Self {
      game_meta,
      game_state,
    } */
  }

  pub fn game_meta(&self) -> &GameMeta {
    &self.game_meta
  }

  pub fn game_state(&self) -> &GameState {
    &self.game_state
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
  pub fn check_move(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    todo!()
  }
  pub fn time_played() -> std::time::Duration {
    todo!()
  }
  pub fn validate_replay(game_meta: &GameMeta) {
    todo!()
  }
}

impl Playable for Greed {
  fn game_field(&self) -> &GameField {
    self.game_state.game_field()
  }
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    todo!()
  }

  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    todo!()
  }

  fn undo_move(&mut self) -> Result<(), GreedError> {
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
          .ok_or(GameFieldParserError::EmptyLine)?,
      )
      .map_err(|cause| GameFieldParserError::InvalidMetaData { cause })?
    } else {
      GameMeta::default()
    };
    let game_field = GameField::try_from(&value[meta_end_pos..])?;
    todo!();
    //Ok(Self {
    //  meta: game_meta,
    //  field: game_field,
    //})
  }
}

impl From<Greed> for String {
  fn from(greed: Greed) -> Self {
    let out = String::with_capacity(1024 + greed.game_field().tile_count());
    todo!("Save game meta then write game field")
  }
}
