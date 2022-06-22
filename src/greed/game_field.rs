use rand::prelude::*;
use std::{
  fmt::{Debug, Display, Write},
  iter::FusedIterator,
  ops::{Index, IndexMut},
};

use super::*;

#[derive(Clone, PartialEq, Eq, Debug)]
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
      let offset = self.row * x_size;
      self.row += 1;
      Some(&self.game_field.vec[offset..offset + x_size])
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
      let offset = self.row * x_size;
      Some(&self.game_field.vec[offset..offset + x_size])
    } else {
      None
    }
  }
}

impl<'a> FusedIterator for RowIter<'a> {}

impl<'a> ExactSizeIterator for RowIter<'a> {}

impl GameField {
  /// Constructs an GameField full of Empty tiles.
  /// It is in an invalid state!
  pub(super) fn new_empty(rows: usize, cols: usize) -> Self {
    Self {
      vec: vec![Tile::EMPTY; rows * cols],
      x_size: cols,
      y_size: rows,
    }
  }

  pub fn default_classic_game_size() -> Pos {
    Pos { x: 79, y: 21 }
  }

  fn pos_to_index(&self, pos: Pos) -> usize {
    pos.x + pos.y * self.x_size
  }

  fn index_to_pos(&self, index: usize) -> Pos {
    let y = index / self.x_size;
    let x = index % self.x_size;
    // let x = self.vec.len() - y * self.x_size;
    Pos { x, y }
  }

  pub fn row_iter(&self) -> RowIter {
    RowIter {
      game_field: self,
      row: 0,
    }
  }

  // pub fn col_iter(&self) -> ColIter {}

  pub(super) fn randomize_field(&mut self, tile_chooser: &mut TileChooser<impl Rng>) {
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

  pub fn tile_count(&self) -> usize {
    self.x_size * self.y_size
  }
}

impl Index<Pos> for GameField {
  type Output = Tile;

  fn index(&self, pos: Pos) -> &Self::Output {
    &self.vec[self.pos_to_index(pos)]
  }
}

impl IndexMut<Pos> for GameField {
  fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
    let pos = self.pos_to_index(pos);
    &mut self.vec[pos]
  }
}

impl From<&GameField> for String {
  fn from(game_field: &GameField) -> Self {
    // Don't forget about the new line characters
    let mut out = String::with_capacity(game_field.tile_count() + game_field.y_size);
    for row in game_field.row_iter() {
      for &tile in row {
        out.push(char::from(tile))
      }
      out.push('\n')
    }
    out
  }
}

impl Display for GameField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in self.row_iter() {
      for &tile in row {
        f.write_char(char::from(tile))?
      }
      f.write_char('\n')?
    }
    Ok(())
  }
}

impl TryFrom<&str> for GameField {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let default_size = GameField::default_classic_game_size();
    let mut vec = Vec::with_capacity(default_size.x * default_size.y);

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
              println!("{x_pos} {x_size}");
              return Err(GameFieldParserError::NotRectangular);
            }
          } else {
            x_size = Some(x_pos);
          }
          x_pos = 0;
          y_pos += 1;
        },
        c => {
          let tile = Tile::try_from(c).map_err(|err| GameFieldParserError::InvalidCharacter {
            found: err.found,
            pos: Pos { x: x_pos, y: y_pos },
          })?;
          if tile == Tile::Player {
            if player_seen {
              return Err(GameFieldParserError::AmbiguousPlayer);
            }
            player_seen = true
          }
          vec.push(tile);
          x_pos += 1;
        },
      }
    }

    if !player_seen {
      return Err(GameFieldParserError::PlayerNotFound);
    }

    if x_pos != 0 {
      return Err(GameFieldParserError::NoTrailingNewLine);
    }

    let x_size = x_size.unwrap();
    assert!(vec.len() == x_size * y_pos);
    let game_field = GameField {
      vec,
      x_size: x_size,
      y_size: y_pos,
    };
    Ok(game_field)
  }
}
