use bitvec::prelude as bv;
use rand::prelude::*;
use std::{
  fmt::{Debug, Display, Write},
  iter::FusedIterator,
  ops::Index,
  slice::SliceIndex,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameField {
  vec: Box<[FakeTile]>,
  x_size: usize,
  y_size: usize,
  /// initial player_pos
  player_pos: Pos,
}

type Amount = u8;

#[derive(Clone, PartialEq, Eq)]
pub struct GameState<'a> {
  mask: bv::BitVec,
  player_pos: Pos,
  moves: Vec<(Direction, Amount)>,
  game_field: &'a GameField,
}

impl<'a> GameState<'a> {
  pub fn new(game_field: &'a GameField) -> Self {
    let player_pos = game_field.player_pos;
    let player_index = game_field.pos_to_index(player_pos).unwrap();
    let mask = bv::BitVec::with_capacity(game_field.tile_count());
    mask.fill(true);
    mask[player_index] = false;

    Self {
      mask,
      game_field,
      moves: Vec::new(),
      player_pos,
    }
  }

  /// TODO TRAITIFY POS_TRAIT
  pub fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    self.game_field.valid_pos(pos)
  }

  /// TODO TRAITIFY POS_TRAIT
  pub fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    self.game_field.pos_to_index(pos)
  }

  /// TODO TRAITIFY POS_TRAIT
  pub fn index_to_pos(&self, index: usize) -> Pos {
    self.game_field.index_to_pos(index)
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

  pub fn check_move(&self, dir: Direction) -> Result<Vec<TileAndIndex>, GreedError> {
    let game_field = self.game_field;

    let mut current_pos = self.player_pos + dir;
    // check if position was valid - is the same as calling dir.valid() obviously
    if current_pos == self.player_pos {
      return Err(GreedError::InvalidDirection);
    }

    let starting_index = game_field
      .pos_to_index(current_pos)
      .ok_or(GreedError::BadMove)?;
    // The first tile the player moves to
    // Tells us how many tiles to move
    let starting_tile = self.get_fake_unchecked(starting_index);
    if starting_tile == FakeTile::EMTPY {
      return Err(GreedError::BadMove);
    }
    let move_amount = starting_tile.amount();

    // TODO: try_collect

    let mut moves = Vec::with_capacity(move_amount.into());
    moves.push((
      // TODO: pos_to_index_unchecked 🙃
      game_field.pos_to_index(self.player_pos).unwrap(),
      Tile::Player,
    ));
    moves.push((starting_index, Tile::from(starting_tile)));
    // collect positions and check for collision -> BadMove
    for _ in 0..move_amount - 1 {
      current_pos += dir;
      let index = game_field
        .pos_to_index(current_pos)
        .ok_or(GreedError::BadMove)?;
      let tile = self.get_fake_unchecked(index);
      if tile == FakeTile::EMTPY {
        return Err(GreedError::BadMove);
      }
      moves.push((index, Tile::from(tile)))
    }

    // return movements
    Ok(moves)
  }

  pub fn move_(
    &mut self,
    game_field: &GameField,
    dir: Direction,
  ) -> Result<Vec<TileAndIndex>, GreedError> {
    // commit movements
    let moves = game_field.check_move(dir)?;

    let mut iter = moves.iter().rev();
    // If check_move is successful the returned vec contains at least one element
    let &(player_index, _) = iter.next().unwrap();
    self.player_pos = self.index_to_pos(player_index);
    for &(index, _) in iter {
      self.mask[index] = false
    }
    Ok(moves)
  }

  pub fn undo_move(&mut self, mut dir: Direction, amount: u8) -> Result<(), GreedError> {
    dir.valid()?;
    dir = !dir; // invert the direction bit flag magic

    todo!()
  }
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

pub type TileAndIndex = (usize, Tile);

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

  pub fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    let x_size = self.x_size as isize;
    let y_size = self.y_size as isize;
    if (0..x_size).contains(&pos.x) && (0..y_size).contains(&pos.y) {
      Some(pos)
    } else {
      None
    }
  }

  pub fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    let x_size = self.x_size as isize;
    let y_size = self.y_size as isize;
    if (0..x_size).contains(&pos.x) && (0..y_size).contains(&pos.y) {
      Some((pos.x + pos.y * x_size) as usize)
    } else {
      None
    }
  }

  pub fn index_to_pos(&self, index: usize) -> Pos {
    let y = (index / self.x_size) as isize;
    let x = (index % self.x_size) as isize;
    // let x = (self.vec.len() - y * self.x_size) as isize;
    Pos { x, y }
  }

  pub fn row_iter(&self) -> RowIter {
    RowIter {
      game_field: self,
      row: 0,
    }
  }

  pub fn tile_iter(&self) -> std::slice::Iter<'_, tile::Tile> {
    self.vec.iter()
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

  /// Can also be interpreted as the maximum score
  pub fn tile_count(&self) -> usize {
    self.x_size * self.y_size
  }

  pub fn score(&self) -> usize {
    self.vec.iter().fold(
      0,
      |accu, &item| {
        if item == Tile::EMPTY {
          accu + 1
        } else {
          accu
        }
      },
    )
  }

  pub fn check_move(&self, dir: Direction) -> Result<Vec<TileAndIndex>, GreedError> {}

  pub fn move_(&mut self, dir: Direction) -> Result<Vec<TileAndIndex>, GreedError> {
    // commit movements
    let moves = self.check_move(dir)?;

    let mut iter = moves.iter().rev();
    // If check_move is successful the returned vec contains at least one element
    let &(player_index, _) = iter.next().unwrap();
    self.vec[player_index] = Tile::Player;
    for &(index, _) in iter {
      self.vec[index] = Tile::EMPTY
    }
    Ok(moves)
  }

  pub fn undo_move(
    &mut self,
    dir: Direction,
    consumed: Vec<TileAndIndex>,
  ) -> Result<(), GreedError> {
    // CHECK FOR INVALID PLAYER TILES IN THE UNDO MOVE
    todo!()
  }
}

pub trait GameFieldGet<I> {
  type Output: ?Sized;

  fn get(&self, index: I) -> Option<&Self::Output>;
}

/// private trait
trait GameFieldGetMut<I> {
  type Output: ?Sized;

  fn get_mut(&mut self, index: I) -> Option<&mut Self::Output>; // TODO SPILT TRAITS MUT REQUIRES type form normal get
}

impl<I> GameFieldGet<I> for GameField
where
  I: SliceIndex<[Tile]>,
{
  type Output = <I as SliceIndex<[Tile]>>::Output;

  fn get(&self, index: I) -> Option<&Self::Output> {
    self.vec.get(index)
  }
}

impl<I> GameFieldGetMut<I> for GameField
where
  I: SliceIndex<[Tile]>,
{
  type Output = <I as SliceIndex<[Tile]>>::Output;

  fn get_mut(&mut self, index: I) -> Option<&mut Self::Output> {
    self.vec.get_mut(index)
  }
}

impl GameFieldGet<Pos> for GameField {
  type Output = Tile;

  fn get(&self, pos: Pos) -> Option<&Self::Output> {
    self.pos_to_index(pos).map(|index| &self.vec[index])
  }
}

impl GameFieldGetMut<Pos> for GameField {
  type Output = Tile;

  fn get_mut(&mut self, pos: Pos) -> Option<&mut Self::Output> {
    self.pos_to_index(pos).map(|index| &mut self.vec[index])
  }
}

impl Index<Pos> for GameField {
  type Output = Tile;

  fn index(&self, pos: Pos) -> &Self::Output {
    &self.vec[self.pos_to_index(pos).unwrap()]
  }
}

impl<I> Index<I> for GameField
where
  I: SliceIndex<[Tile]>,
{
  type Output = <I as SliceIndex<[Tile]>>::Output;

  fn index(&self, pos: I) -> &Self::Output {
    &self.vec[pos]
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
    let game_field = GameField {
      vec,
      x_size,
      y_size: y_pos,
    };
    Ok(game_field)
  }
}
