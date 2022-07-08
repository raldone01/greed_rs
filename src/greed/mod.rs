mod game;
#[allow(unused_imports)]
use game::*;
pub use game::{GameMeta, Greed};

pub mod greed_error;
/// Make errors available internally
use greed_error::*;

mod tile;
pub use tile::Tile;

mod pos;
pub use pos::Pos;

mod size_2d;
pub use size_2d::Size2D;

mod game_field;
pub use game_field::GameField;

mod game_state;
pub use game_state::GameState;

mod playable;
pub use playable::Playable;

mod tile_grid;
pub use tile_grid::{
  ColIterator, RowIterator, StrideTileIterator, TileGet, TileGrid, TileIterator,
};

mod tile_chooser;
/// Internal
use tile_chooser::{DifficultyMap, DifficultyMapExt, TileChooser};

mod fake_tile;
/// Internal
use fake_tile::FakeTile;

mod direction;
pub use direction::Direction;

#[cfg(test)]
mod test;
