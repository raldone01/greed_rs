use bitvec::prelude as bv;
use rand::prelude::*;
use std::{
  fmt::{Debug, Display},
  iter::FusedIterator,
  ops::Index,
};
use thiserror::Error;

use super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
struct FakeTile {
  amount: u8,
}

impl FakeTile {
  const EMTPY: FakeTile = FakeTile { amount: 0 };

  pub fn amount(self) -> u8 {
    self.amount
  }
}

#[derive(Error, Debug, PartialEq)]
#[error("Can't convert player Tile to FakeTile")]
pub struct FakeTileConversionError {}

impl TryFrom<Tile> for FakeTile {
  type Error = FakeTileConversionError;

  fn try_from(value: Tile) -> Result<Self, Self::Error> {
    let amount = value.amount().ok_or(FakeTileConversionError {})?;
    Ok(FakeTile { amount })
  }
}

impl From<FakeTile> for Tile {
  fn from(fake_tile: FakeTile) -> Self {
    Tile::try_from(fake_tile.amount).unwrap()
  }
}

impl Debug for FakeTile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", Tile::from(*self))
  }
}

/// This immutable structure represents the initial state of a game of greed.
/// It contains all tiles including the player.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameField {
  vec: Box<[FakeTile]>,
  x_size: usize,
  y_size: usize,
  /// initial player_pos
  player_pos: Pos,
}

type Amount = u8;
pub trait TileGet<I> {
  fn get(&self, index: I) -> Option<Tile>;
  fn get_unchecked(&self, index: I) -> Tile;
}

pub struct TileIterator<'a, T: TileGrid + ?Sized> {
  index: usize,
  grid: &'a T,
}

impl<'a, T: TileGrid + ?Sized> Iterator for TileIterator<'a, T> {
  type Item = Tile;

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.index;
    if index < self.grid.tile_count() {
      self.index = index + 1;
      Some(self.grid[index])
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let count = self.grid.tile_count();
    (count, Some(count))
  }
}
impl<'a, T: TileGrid + ?Sized> DoubleEndedIterator for TileIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let index = self.index;
    if index > 0 {
      self.index = index - 1;
      Some(self.grid[index])
    } else {
      None
    }
  }
}

impl<'a, T: TileGrid + ?Sized> FusedIterator for TileIterator<'a, T> {}
impl<'a, T: TileGrid + ?Sized> ExactSizeIterator for TileIterator<'a, T> {}
// impl<'a, T: TileGrid + ?Sized> RandomAccessIterator for TileIterator<'a, T> {}

pub trait TileGrid: TileGet<usize> + TileGet<Pos> {
  /// For the default implementations to work the each
  /// value in the returned tuple must not exceed isize::MAX.
  fn dimensions(&self) -> (usize, usize);

  fn player_pos(&self) -> Pos;

  /// Can also be interpreted as the maximum score
  fn tile_count(&self) -> usize {
    let (x_size, y_size) = self.dimensions();
    x_size * y_size
  }
  fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    let (x_size, y_size) = self.dimensions();
    let x_size = x_size as isize;
    let y_size = y_size as isize;
    if (0..x_size).contains(&pos.x) && (0..y_size).contains(&pos.y) {
      Some(pos)
    } else {
      None
    }
  }
  /// Warning: This function can only catch if the index is out of bounds!
  fn valid_index(&self, index: usize) -> Option<usize> {
    let tile_count = self.tile_count();
    if index < tile_count {
      Some(index)
    } else {
      None
    }
  }
  fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    let pos = self.valid_pos(pos)?;
    Some(self.pos_to_index_unchecked(pos))
  }
  /// Assumes that the position is valid.
  fn pos_to_index_unchecked(&self, pos: Pos) -> usize {
    let (x_size, y_size) = self.dimensions();
    pos.x as usize + (pos.y as usize) * x_size
  }
  fn index_to_pos(&self, index: usize) -> Option<Pos> {
    let index = self.valid_index(index)?;
    Some(self.index_to_pos_unchecked(index))
  }
  /// Assumes that the index is valid
  fn index_to_pos_unchecked(&self, index: usize) -> Pos {
    let (x_size, y_size) = self.dimensions();
    let y = (index / x_size) as isize;
    let x = (index % x_size) as isize;
    // let x = (x_size * y_size - y * x_size) as isize;
    Pos { x, y }
  }

  fn tile_iter<'a>(&'a self) -> TileIterator<'a, Self> {
    TileIterator {
      index: 0,
      grid: self,
    }
  }

  fn score(&self) -> usize {
    self.tile_iter().fold(
      0,
      |accu, item| {
        if item == Tile::EMPTY {
          accu + 1
        } else {
          accu
        }
      },
    )
  }
}

/// This mutable structure represents a modified game field.
/// It encodes which fields have been consumed and where the player pos.
#[derive(Clone, PartialEq, Eq)]
pub struct GameState<'a> {
  /// If a tile has a false mask it should be considered as an empty tile.
  /// The player also has a false mask.
  mask: bv::BitVec,
  player_pos: Pos,
  moves: Vec<(Direction, Amount)>,
  game_field: &'a GameField,
}

impl<'a> GameState<'a> {
  pub fn new(game_field: &'a GameField) -> Self {
    let player_pos = game_field.player_pos;
    let player_index = game_field.pos_to_index(player_pos).unwrap();
    let mut mask = bv::BitVec::with_capacity(game_field.tile_count());
    mask.fill(true);
    // https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html#writing-into-a-bit-vector
    mask.set(player_index, false);

    Self {
      mask,
      game_field,
      moves: Vec::new(),
      player_pos,
    }
  }

  pub fn game_field(&self) -> &'a GameField {
    self.game_field
  }

  fn get_fake_unchecked(&self, index: usize) -> FakeTile {
    if self.mask[index] == false {
      FakeTile { amount: 0 }
    } else {
      self.game_field.vec[index]
    }
  }

  /// Checks if a move would be valid.
  /// Returns the indices that would be consumed including the old player pos and the new player pos.
  /// So `ret.unwrap().len()-1` would be the amount of tiles consumed.
  /// To get which tiles would be consumed use the game_field or this unmodified game_state to look them up.
  /// You can use any game_field or game_state with the same dimensions to convert the index to a position.
  pub fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    let mut current_pos = self.player_pos + dir;
    // check if position was valid - is the same as calling dir.valid() obviously
    if current_pos == self.player_pos {
      return Err(GreedError::InvalidDirection);
    }

    let starting_index = self.pos_to_index(current_pos).ok_or(GreedError::BadMove)?;
    // The first tile the player moves to
    // Tells us how many tiles to move
    let starting_tile = self.get_fake_unchecked(starting_index);
    if starting_tile == FakeTile::EMTPY {
      return Err(GreedError::BadMove);
    }
    let move_amount = starting_tile.amount();

    // TODO: try_collect

    let mut moves = Vec::with_capacity(move_amount.into());
    // first push the old player pos
    moves.push(
      // TODO: pos_to_index_unchecked 🙃 or just unwrap_unchecked
      self.pos_to_index(self.player_pos).unwrap(),
    );
    // push the tile that gave us the amount
    moves.push(starting_index);
    // collect positions and check for collision -> BadMove
    for _ in 1..move_amount {
      current_pos += dir;
      let index = self.pos_to_index(current_pos).ok_or(GreedError::BadMove)?;
      let tile = self.get_fake_unchecked(index);
      if tile == FakeTile::EMTPY {
        return Err(GreedError::BadMove);
      }
      moves.push(index)
    }

    // return movements
    Ok(moves)
  }

  // TODO: impl TileGrid for GameState {}
  // TODO: impl TileGrid for GameField {}

  /// TODO: give the user a more efficient way to execute a move checked by check_move?
  /// Maybe introduce a CheckedMove type to do that safely?
  ///
  /// For the return see check_move function.
  pub fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError> {
    // commit movements
    let moves = self.check_move(dir)?;

    let mut iter = moves.iter().rev();
    // If check_move is successful the returned vec contains at least one element
    let &player_index = iter.next().unwrap();
    self.player_pos = self.index_to_pos(player_index);
    for &index in iter {
      self.mask.set(index, false)
    }

    // update the moves array
    self
      .moves
      // TODO: unwrap_unchecked
      .push((dir, u8::try_from(moves.len() - 1).unwrap()));
    Ok(moves)
  }

  pub fn undo_move(&mut self) -> Result<(), GreedError> {
    let last_move = self.moves.pop().ok_or(GreedError::BadMove)?;
    let (dir, amount) = last_move;
    // dir.valid()?; // always valid we control the moves
    let dir = !dir; // invert the direction bit flag magic

    // 1..amount because we don't want to uncheck the player
    for _ in 1..amount {
      let index = self.pos_to_index(self.player_pos).unwrap(); // TODO: unsafe_unwrap
      self.mask.set(index, true);
      self.player_pos += dir;
    }
    // move the player pos without setting the mask to true
    self.player_pos += dir;

    Ok(())
  }
}

impl GameField {
  /// Constructs an GameField full of Empty tiles.
  /// It is in an invalid state!
  pub(super) fn new_empty(rows: usize, cols: usize) -> Self {
    todo!()
    /* Self {
      vec: vec![Tile::EMPTY; rows * cols],
      x_size: cols,
      y_size: rows,
    } */
  }

  pub fn new_game_state(&self) -> GameState {
    todo!()
  }

  pub fn default_classic_game_dimensions() -> Pos {
    Pos { x: 79, y: 21 }
  }

  pub(super) fn randomize_field(&mut self, tile_chooser: &mut TileChooser<impl Rng>) {
    todo!()
    /* for tile in self.vec.iter_mut() {
      *tile = tile_chooser.choose();
    }
    let pp = tile_chooser.rng.gen_range(0..self.vec.len());
    self.vec[pp] = Tile::Player; */
  }
}

/*
/// Would give users the ability to insert multiple players or to remove the player!
impl IndexMut<Pos> for GameField {
  fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
    let pos = self.pos_to_index(pos);
    &mut self.vec[pos]
  }
} */

impl From<&GameField> for String {
  fn from(game_field: &GameField) -> Self {
    // Don't forget about the new line characters
    let mut out = String::with_capacity(game_field.tile_count() + game_field.y_size);
    /* for row in game_field.row_iter() {
      for &tile in row {
        out.push(char::from(tile))
      }
      out.push('\n')
    } */
    todo!();
    out
  }
}

impl Display for GameField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    /* for row in self.row_iter() {
      for &tile in row {
        f.write_char(char::from(tile))?
      }
      f.write_char('\n')?
    } */
    todo!();
    Ok(())
  }
}

impl TryFrom<&str> for GameField {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let default_size = GameField::default_classic_game_dimensions();
    let mut vec = Vec::with_capacity(usize::try_from(default_size.x * default_size.y).unwrap());

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
          let tile = Tile::try_from(c).map_err(|err| GameFieldParserError::InvalidCharacter {
            found: err.found,
            pos: Pos {
              x: isize::try_from(x_pos).unwrap(),
              y: isize::try_from(y_pos).unwrap(),
            },
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
    todo!()
    /*let game_field = GameField {
      vec,
      x_size,
      y_size: y_pos,
    };
    Ok(game_field) */
  }
}
impl TileGrid for GameField {
  fn dimensions(&self) -> (usize, usize) {
    (self.x_size, self.y_size)
  }

  fn player_pos(&self) -> Pos {
    self.player_pos
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
