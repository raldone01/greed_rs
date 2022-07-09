use super::*;
use chrono::{DateTime, Utc};
use rand::distributions::Uniform;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use sha2::{Digest, Sha512};
use std::{
  rc::Rc,
  time::{Duration, Instant},
};

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
  #[serde(default)]
  pub time_spent_ms: i64,
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
  pub inital_game_field: Option<GameField>,
}

impl GameMeta {
  pub fn new(greed: &Greed) -> Self {
    let utc_started_ms = greed
      .started_instant
      .map(|instant| instant.timestamp_millis());
    let utc_finished_ms = greed
      .finished_instant
      .map(|instant| instant.timestamp_millis());
    Self {
      file_version: Some(1),
      greed_version: Some(1),
      seed: greed.seed.clone(),
      name: Some(greed.name.clone()),
      utc_started_ms,
      utc_finished_ms,
      time_spent_ms: greed
        .time_spent()
        .as_millis()
        .try_into()
        .expect("How the hell did you play that long? (Create an issue)"),
      difficulty_map: greed.difficulty_map.clone(),
      moves: Some(
        greed
          .game_state
          .moves()
          .iter()
          .map(|(dir, _)| *dir)
          .collect(),
      ),
      score: Some(greed.score()),
      human_score: Some(greed.human_score()),
      undos: Some(greed.undos),
      inital_game_field: Some(greed.game_field().clone()),
    }
  }
}

pub struct GreedBuilder {
  seed: Option<String>,
  name: Option<String>,
  difficulty_map: Option<DifficultyMap>,
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

    let name = self.name.clone().unwrap_or_else(|| seed.clone());

    let difficulty_map = self
      .difficulty_map
      .clone()
      .unwrap_or_else(|| DifficultyMap::default_difficulties().clone());

    Greed::new_from_builder(self.size, seed, name, difficulty_map)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Greed {
  /// None if the game is custom game and the seed is unkown.
  seed: Option<String>,
  /// We will just pick a name if we don't get one.
  name: String,
  /// None if the game was loaded from a string with no starting time.
  started_instant: Option<DateTime<Utc>>,
  finished_instant: Option<DateTime<Utc>>,
  started_session: Instant,
  time_spent: Duration,
  /// None if the game is custom game.
  difficulty_map: Option<DifficultyMap>,
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
    hasher.update(&string_seed);
    let hash = hasher.finalize();
    let used_hash = <[u8; 16]>::try_from(&hash[0..16]).unwrap();
    // init the random gen with the first 16 bytes of the hash
    let mut rng = rand_pcg::Pcg64Mcg::from_seed(used_hash);
    #[allow(clippy::or_fun_call)] // TODO: Create an issue
    let mut tile_chooser = TileChooser::new(&mut rng, &difficulty_map);
    let game_field = Rc::from(GameField::new_random(&mut tile_chooser, size));

    Greed {
      seed: Some(string_seed),
      name,
      started_instant: Some(chrono::Utc::now()),
      finished_instant: None,
      started_session: Instant::now(),
      time_spent: Duration::new(0, 0),
      difficulty_map: Some(difficulty_map),
      undos: 0,
      game_state: GameState::new(game_field),
    }
  }

  pub fn game_meta(&self) -> GameMeta {
    GameMeta::new(self)
  }

  pub fn game_state(&self) -> &GameState {
    &self.game_state
  }

  pub fn session_time(&self) -> std::time::Duration {
    Instant::now() - self.started_session
  }

  pub fn time_spent(&self) -> std::time::Duration {
    self.time_spent + self.session_time()
  }

  pub fn total_move_count(&self) -> usize {
    self.game_state.move_count() + self.undos
  }

  pub fn undo_count(&self) -> usize {
    self.undos
  }

  pub fn validate_replay(game_meta: &GameMeta) {
    todo!()
  }

  pub fn human_score(&self) -> usize {
    todo!()
  }
}

impl Playable for Greed {
  fn game_field(&self) -> &GameField {
    self.game_state.game_field()
  }

  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    self.game_state.check_move(dir)
  }

  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    self.game_state.move_(dir)
  }

  fn undo_move(&mut self) -> Result<(), GreedError> {
    self.game_state.undo_move().map(|_| self.undos += 1)
  }

  fn move_count(&self) -> usize {
    self.game_state.moves().len()
  }
}

impl<T> TileGet<T> for Greed
where
  GameState: TileGet<T>,
{
  fn get(&self, index: T) -> Option<Tile> {
    self.game_state.get(index)
  }

  fn get_unchecked(&self, index: T) -> Tile {
    self.game_state.get_unchecked(index)
  }
}

impl TileGrid for Greed {
  fn dimensions(&self) -> Size2D {
    self.game_field().dimensions()
  }
  fn player_pos(&self) -> Pos {
    self.game_field().player_pos()
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
