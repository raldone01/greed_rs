#[allow(clippy::module_inception)]
mod greed;
pub use greed::{GameMeta, Greed};

pub mod greed_error;
/// Make errors available internally
use greed_error::{
  GameFieldParserError, GreedParserError, MoveValidationError, PlayableError, ReproductionError,
  TileParseError,
};

mod tile;
pub use tile::Tile;

mod pos;
pub use pos::Pos;

mod size_2d;
pub use size_2d::{Size2D, Size2DConversionError};

mod game_field;
pub use game_field::GameField;

mod game_state;
pub use game_state::{GameState, GameStateRebuildFromDiffError};

mod playable;
pub use playable::Playable;

mod tile_grid;
pub use tile_grid::{
  ColIterator, RowIterator, StrideTileIterator, TileGet, TileGrid, TileIterator,
};
mod tile_probs;
pub use tile_probs::{TileProbs, TileProbsConversionError};

mod seed;
pub use seed::{Seed, SeedConversionError, UserString, UserStringError};

mod tile_chooser;
/// Internal
use tile_chooser::TileChooser;

mod fake_tile;
/// Internal
use fake_tile::{FakeTile, FakeTileConversionError};

mod amount;
pub use amount::Amount;

mod direction;
pub use direction::Direction;

mod greed_builder;
pub use greed_builder::GreedBuilder;

#[cfg(test)]
mod test;
