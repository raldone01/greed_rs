use super::{
  Amount, Direction, GameField, GameState, GreedParserError, MoveValidationError, Playable,
  PlayableError, Pos, ReproductionError, Seed, Size2D, Tile, TileGet, TileGrid, Grid2D
};
use chrono::{DateTime, Local, TimeZone, Utc};
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
  pub initial_game_field: Option<GameField>,
  pub last_game_field: Option<GameField>,
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
      initial_game_field: Some(greed.game_field().clone()),
      last_game_field: Some(greed.game_state.to_game_field()),
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

  /// Accepts either GameMeta as Json5 or one GameField-String and creates a Greed instance from it.
  pub fn load_from_string(str: &str) -> Result<Greed, GreedParserError> {
    // load the meta data if available

    #[allow(clippy::iter_nth_zero)]
    let first_char = str.chars().nth(0).ok_or(GreedParserError::EmptyString)?;
    let game_meta = if first_char == '{' {
      json5::from_str::<GameMeta>(str)
        .map_err(|cause| GreedParserError::InvalidMetaDataFromat { cause })?
    } else {
      // Create default game_meta and set initial_field
      GameMeta {
        initial_game_field: Some(
          GameField::try_from(str)
            .map_err(|cause| GreedParserError::GameFieldParserError { cause })?,
        ),
        ..Default::default()
      }
    };

    // assemble the game_field
    let game_field = Rc::from(
      game_meta
        .initial_game_field
        .or_else(|| game_meta.last_game_field.clone())
        .or_else(|| game_meta.seed.as_ref().map(GameField::from_seed))
        .ok_or(GreedParserError::MissingGameFieldInformation)?,
    );

    let moves = game_meta.moves.unwrap_or_default();

    let game_state = if let Some(last_game_field) = game_meta.last_game_field {
      GameState::try_rebuild_from_game_field_diff(game_field, &last_game_field, moves)?
    } else {
      // reconstruct the game_state by applying all moves to the inital_game_state
      let mut game_state = GameState::new(game_field);
      for move_ in moves {
        game_state.move_(move_.0)?;
      }
      game_state
    };

    // get the game name
    let name = game_meta
      .name
      .or_else(|| game_meta.seed.as_ref().map(String::from))
      .unwrap_or_else(|| {
        Local::now()
          .format("Custom Game from %d/%b/%Y %H:%M:%S")
          .to_string()
      });

    Ok(Self {
      seed: game_meta.seed,
      name,
      started_instant: game_meta
        .utc_started_ms
        .map(|utc_started_ms| Utc.timestamp_millis(utc_started_ms)),
      finished_instant: game_meta
        .utc_finished_ms
        .map(|utc_finished_ms| Utc.timestamp_millis(utc_finished_ms)),
      started_session: Instant::now(),
      time_spent: Duration::from_millis(
        game_meta
          .time_spent_ms
          .try_into()
          .map_err(|cause| GreedParserError::InvalidDuration { cause })?,
      ),
      undos: game_meta.undos.unwrap_or(0),
      game_state,
    })
  }

  pub fn save_to_string(&self) -> String {
    let meta = GameMeta::new(self);
    json5::to_string(&meta).unwrap()
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
  fn player_pos(&self) -> Pos {
    self.game_state.player_pos()
  }

  // The following functions are implemented as wrappers to make sure they aren't generated again
  fn score(&self) -> usize {
    self.game_state.score()
  }
}

impl Grid2D for Greed {
  fn dimensions(&self) -> Size2D {
    self.game_state.dimensions()
  }

  // The following functions are implemented as wrappers to make sure they aren't generated again
  fn tile_count(&self) -> usize {
    self.game_state.tile_count()
  }

  fn is_valid_pos(&self, pos: Pos) -> bool {
    self.game_state.is_valid_pos(pos)
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
}
