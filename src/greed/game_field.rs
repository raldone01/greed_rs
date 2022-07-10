use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use super::{
  FakeTile, FakeTileConversionError, GameFieldParserError, Pos, Size2D, Tile, TileChooser, TileGet,
  TileGrid,
};

/// This immutable structure represents the initial state of a game of greed.
/// It contains all tiles including the player.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct GameField {
  pub(super) vec: Box<[FakeTile]>,
  size: Size2D,
  /// initial player_pos
  player_pos: Pos,
}

impl TileGrid for GameField {
  fn dimensions(&self) -> Size2D {
    self.size
  }

  fn player_pos(&self) -> Pos {
    self.player_pos
  }
}

impl GameField {
  pub fn default_classic_game_dimensions() -> Size2D {
    Size2D::new(79, 21)
  }

  pub(super) fn new_random(tile_chooser: &mut TileChooser<impl Rng>, size: Size2D) -> Self {
    let vec = (0..size.tile_count())
      .map(|_| tile_chooser.choose())
      .collect();

    let player_pos = Pos {
      x: tile_chooser.rng.gen_range(0..size.x_size) as isize,
      y: tile_chooser.rng.gen_range(0..size.y_size) as isize,
    };
    Self {
      vec,
      size,
      player_pos,
    }
  }
}

impl From<&GameField> for String {
  fn from(game_field: &GameField) -> Self {
    game_field.into_string()
  }
}

impl TryFrom<&str> for GameField {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let default_size = GameField::default_classic_game_dimensions();
    let mut vec = Vec::with_capacity(default_size.tile_count());

    let mut x_size = None;
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut player_pos = None;
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
          // This technically doesn't need to be always calculated, but its po
          let pos = {
            Pos::new(
              x_pos
                .try_into()
                .map_err(|_| GameFieldParserError::InvalidSize)?,
              y_pos
                .try_into()
                .map_err(|_| GameFieldParserError::InvalidSize)?,
            )
          };
          let tile = Tile::try_from(c).map_err(|err| GameFieldParserError::InvalidCharacter {
            found: err.found,
            pos,
          })?;

          match FakeTile::try_from(tile) {
            Ok(tile) => vec.push(tile),
            Err(FakeTileConversionError::PlayerTile) => {
              if let Some(first) = player_pos {
                return Err(GameFieldParserError::AmbiguousPlayer { first, second: pos });
              }
              player_pos = Some(pos);
              vec.push(FakeTile::EMTPY);
            },
          }

          x_pos += 1;
        },
      }
    }

    if y_pos == 0 {
      return Err(GameFieldParserError::InvalidSize);
    }

    if x_pos != 0 {
      return Err(GameFieldParserError::NoTrailingNewLine);
    }

    let size = Size2D::new(x_size.unwrap(), y_pos);
    assert!(vec.len() == size.tile_count());
    let vec = vec.into_boxed_slice();

    let game_field = GameField {
      vec,
      size,
      player_pos: player_pos.ok_or(GameFieldParserError::PlayerNotFound)?,
    };
    Ok(game_field)
  }
}

impl TileGet<usize> for GameField {
  fn get(&self, index: usize) -> Option<Tile> {
    // player_pos is always valid(Hopefully)
    if index == self.pos_to_index_unchecked(self.player_pos) {
      Some(Tile::Player)
    } else {
      // Never masked since we are GF

      Some(Tile::from(*self.vec.get(index)?))
    }
  }
  fn get_unchecked(&self, index: usize) -> Tile {
    // player_pos is always valid(Hopefully)
    if index == self.pos_to_index_unchecked(self.player_pos) {
      Tile::Player
    } else {
      // Never masked since we are GF

      Tile::from(self.vec[index])
    }
  }
}

impl TileGet<Pos> for GameField {
  fn get(&self, pos: Pos) -> Option<Tile> {
    if pos == self.player_pos {
      Some(Tile::Player)
    } else {
      // Never masked since we are GF

      let index = self.pos_to_index(pos)?;
      Some(Tile::from(*self.vec.get(index)?))
    }
  }
  fn get_unchecked(&self, pos: Pos) -> Tile {
    if pos == self.player_pos {
      Tile::Player
    } else {
      // Never masked since we are GF

      let index = self.pos_to_index_unchecked(pos);
      Tile::from(self.vec[index])
    }
  }
}

impl Display for GameField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.display_fmt(f)
  }
}
