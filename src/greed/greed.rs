use super::{
  Amount, Direction, GameField, GameState, GreedParserError, MoveValidationError, Playable,
  PlayableError, Pos, ReproductionError, Seed, Size2D, Tile, TileGet, TileGrid,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{
  rc::Rc,
  time::{Duration, Instant},
};

#[skip_serializing_none]
#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct GameMeta {
  pub file_version: Option<u64>,
  pub greed_version: Option<u64>,
  pub seed: Option<Seed>,
  pub name: Option<String>,
  pub utc_started_ms: Option<i64>,
  pub utc_finished_ms: Option<i64>,
  #[serde(default)]
  pub time_spent_ms: i64,
  pub moves: Option<Vec<(Direction, Amount)>>,
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
      moves: Some(greed.game_state.moves().to_vec()),
      score: Some(greed.score()),
      human_score: Some(greed.human_score()),
      undos: Some(greed.undos),
      inital_game_field: Some(greed.game_field().clone()),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Greed {
  /// None if the game is custom game and the seed is unkown.
  seed: Option<Seed>,
  /// We will just pick a name if we don't get one.
  name: String,
  /// None if the game was loaded from a string with no starting time.
  started_instant: Option<DateTime<Utc>>,
  finished_instant: Option<DateTime<Utc>>,
  started_session: Instant,
  time_spent: Duration,
  undos: usize,
  game_state: GameState,
}

impl Greed {
  pub(super) fn new_from_builder(name: String, seed: Seed) -> Self {
    let game_field = Rc::from(GameField::from_seed(&seed));

    Greed {
      seed: Some(seed),
      name,
      started_instant: Some(chrono::Utc::now()),
      finished_instant: None,
      started_session: Instant::now(),
      time_spent: Duration::new(0, 0),
      undos: 0,
      game_state: GameState::new(game_field),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn seed(&self) -> Option<&Seed> {
    self.seed.as_ref()
  }

  // pub fn load_from_reader() {}
  // pub fn save_to_writer() {}
  #[allow(unused_variables)]
  pub fn load_from_string(str: &str) -> Result<Greed, GreedParserError> {
    // load the meta data if available
    // Finds the last index of } which must indicate the end of the meta data
    let meta_end_pos = str
      .char_indices()
      .rev()
      .find(|&(_, char)| char == '}')
      // +1 to include } and +1 to include \n
      .map_or(0, |(index, _)| index + 2);
    let game_meta = if meta_end_pos != 0 {
      json5::from_str::<GameMeta>(
        str
          .get(0..meta_end_pos)
          .ok_or(GreedParserError::LeadingEmptyLine)?,
      )
      .map_err(|cause| GreedParserError::InvalidMetaDataFromat { cause })?
    } else {
      GameMeta::default()
    };

    // load last_game_field if available
    let game_field_str = &str[meta_end_pos..];
    let last_game_field = if !game_field_str.is_empty() {
      Some(
        GameField::try_from(game_field_str)
          .map_err(|cause| GreedParserError::GameFieldParserError { cause })?,
      )
    } else {
      None
    };

    // load inital_game_field
    let inital_game_field = game_meta.inital_game_field.clone().unwrap();
    // TODO: .or_else(|| game_meta.seed.map(|seed| GameField::from_seed(seed)));

    // if moves and inital_game_field -> gen last_game_field ff
    // if moves and seed -> gen last_game_field ff

    // if conflicting last_game_field and initial_game_field?
    // if last_game_field and inital_game_field then compute mask?

    let game_field = game_meta
      .inital_game_field
      .ok_or_else(|| GameField::try_from(game_field_str));

    //let name = game_meta
    //  .name
    //  .or_else(|| game_meta.seed.clone())
    //  .unwrap_or_else(|| {
    //    Local::now()
    //      .format("Custom Game from %d/%b/%Y %H:%M:%S")
    //      .to_string()
    //  });

    todo!();
    //Ok(Self {
    //  seed: game_meta.seed,
    //  name,
    //  started_instant: game_meta
    //    .utc_started_ms
    //    .map(|utc_started_ms| Utc.timestamp_millis(utc_started_ms)),
    //  finished_instant: game_meta
    //    .utc_finished_ms
    //    .map(|utc_finished_ms| Utc.timestamp_millis(utc_finished_ms)),
    //  started_session: Instant::now(),
    //  time_spent: Duration::from_millis(
    //    game_meta
    //      .time_spent_ms
    //      .try_into()
    //      .map_err(|cause| GreedParserError::InvalidDuration { cause })?,
    //  ),
    //  difficulty_map: game_meta.difficulty_map,
    //  undos: game_meta.undos.unwrap_or(0),
    //  game_state: GameState::new_with_moves(
    //    Rc::new(game_field.unwrap()), // TODO: remove unwrap
    //    game_meta.moves.unwrap_or_else(|| Vec::new()),
    //  ),
    //})
  }

  pub fn save_to_string(&self) -> String {
    let meta = GameMeta::new(self);
    let mut str = json5::to_string(&meta).unwrap();
    str.push('\n');
    str += &String::from(self.game_state());
    str
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

  /// Validates the moves array
  pub fn validate_moves() -> Result<(), MoveValidationError> {
    todo!()
  }

  /// Validates if the seed reproduces the saved game state and checks that all moves are valid.
  /// Also uses a difficulty map if available.
  pub fn validate_reproducibility() -> Result<(), ReproductionError> {
    todo!()
  }

  /// TODO: Returns 0 for now
  pub fn human_score(&self) -> usize {
    0
  }
}

impl Playable for Greed {
  fn game_field(&self) -> &GameField {
    self.game_state.game_field()
  }

  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, PlayableError> {
    self.game_state.check_move(dir)
  }

  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, PlayableError> {
    self.game_state.move_(dir)
  }

  fn undo_move(&mut self) -> Result<(), PlayableError> {
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
    self.game_state.dimensions()
  }
  fn player_pos(&self) -> Pos {
    self.game_state.player_pos()
  }

  // The following functions are implemented as wrappers to make sure they aren't generated again
  fn tile_count(&self) -> usize {
    self.game_state.tile_count()
  }

  fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    self.game_state.valid_pos(pos)
  }

  fn valid_index(&self, index: usize) -> Option<usize> {
    self.game_state.valid_index(index)
  }

  fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    self.game_state.pos_to_index(pos)
  }

  fn pos_to_index_unchecked(&self, pos: Pos) -> usize {
    self.game_state.pos_to_index_unchecked(pos)
  }

  fn index_to_pos(&self, index: usize) -> Option<Pos> {
    self.game_state.index_to_pos(index)
  }

  fn index_to_pos_unchecked(&self, index: usize) -> Pos {
    self.game_state.index_to_pos_unchecked(index)
  }

  fn score(&self) -> usize {
    self.game_state.score()
  }
}

impl From<Greed> for String {
  fn from(greed: Greed) -> Self {
    let _out = String::with_capacity(1024 + greed.game_field().tile_count());
    todo!("Save game meta then write game field")
  }
}