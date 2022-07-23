use super::{
  Amount, Direction, FakeTile, GameField, Grid2D, Playable, PlayableError, Pos, Size2D, Tile,
  TileGet, TileGrid,
};
use bitvec::prelude as bv;
use std::{
  fmt::{Debug, Display},
  rc::Rc,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GameStateRebuildFromDiffError {
  #[error(
    "Tiles don't match at {} - inital tile: {}; last tile {}",
    pos,
    initial_tile,
    last_tile
  )]
  InconsistentTiles {
    pos: Pos,
    initial_tile: Tile,
    last_tile: Tile,
  },
  #[error("Dimensions not equal: inital size {initial_size} and last size {last_size}")]
  DimensionsNotEqual {
    initial_size: Size2D,
    last_size: Size2D,
  },
}

/// This mutable structure represents a modified game field.
/// It encodes which fields have been consumed and the player pos.
#[derive(Clone, PartialEq, Eq)]
pub struct GameState {
  /// If a tile has a false mask it should be considered as an empty tile.
  /// The player also has a false mask.
  mask: bv::BitVec,
  player_pos: Pos,
  moves: Vec<(Direction, Amount)>,
  game_field: Rc<GameField>,
}

impl GameState {
  pub(super) fn new_with_moves(game_field: Rc<GameField>, moves: Vec<(Direction, Amount)>) -> Self {
    let player_pos = game_field.player_pos();
    let player_index = game_field.pos_to_index(player_pos).unwrap();
    let tile_count = game_field.tile_count();

    let mut mask = bv::BitVec::with_capacity(tile_count);
    mask.resize(tile_count, true);
    // https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html#writing-into-a-bit-vector
    mask.set(player_index, false);

    // apply already empty tiles from game_field to mask - sometimes avoids 2 deep lookups
    for it in 0..tile_count {
      let last_tile = game_field.vec[it];
      if last_tile == FakeTile::EMTPY {
        mask.set(it, false);
      }
    }

    Self {
      mask,
      game_field,
      moves,
      player_pos,
    }
  }

  pub fn new(game_field: Rc<GameField>) -> Self {
    Self::new_with_moves(game_field, Vec::new())
  }

  pub(super) fn try_rebuild_from_game_field_diff(
    initial_game_field: Rc<GameField>,
    last_game_field: &GameField,
    moves: Vec<(Direction, Amount)>,
  ) -> Result<Self, GameStateRebuildFromDiffError> {
    let initial_size = initial_game_field.dimensions();
    let last_size = last_game_field.dimensions();
    if initial_size != last_size {
      Err(GameStateRebuildFromDiffError::DimensionsNotEqual {
        initial_size,
        last_size,
      })?
    }

    let player_pos = last_game_field.player_pos();
    let player_index = last_game_field.pos_to_index_unchecked(player_pos);

    let tile_count = initial_size.tile_count();
    let mut mask = bv::BitVec::with_capacity(tile_count);
    mask.resize(tile_count, true);
    mask.set(player_index, false);

    for it in 0..tile_count {
      let initial_tile = initial_game_field.vec[it];
      let last_tile = last_game_field.vec[it];
      if last_tile == FakeTile::EMTPY {
        mask.set(it, false);
      } else if initial_tile != last_tile {
        Err(GameStateRebuildFromDiffError::InconsistentTiles {
          pos: last_game_field.index_to_pos_unchecked(it),
          initial_tile: Tile::from(initial_tile),
          last_tile: Tile::from(last_tile),
        })?
      }
    }

    Ok(Self {
      mask,
      player_pos,
      moves,
      game_field: initial_game_field,
    })
  }

  pub fn moves(&self) -> &[(Direction, Amount)] {
    &self.moves
  }

  pub(super) fn get_fake_unchecked(&self, index: usize) -> FakeTile {
    #[allow(clippy::bool_comparison)]
    // self.mask[index] == false is purposefully used over !self.mask[index] i
    if self.mask[index] == false {
      FakeTile::EMTPY
    } else {
      self.game_field.vec[index]
    }
  }

  fn move_set(&mut self, pos: Pos, dir: Direction, amount: u8, mask: bool) {
    let mut pos = pos;

    for _ in 0..amount {
      let index = self.pos_to_index_unchecked(pos);
      self.mask.set(index, mask);
      pos += dir;
    }
  }

  /// Creates a new game_field from the current game_state.
  /// Warning: Discards tile information of the cleared tiles.
  pub fn to_game_field(&self) -> GameField {
    // GameField::try_from(String::from(self).as_str()).unwrap()
    GameField::new_from_game_state(self)
  }
}

impl TileGrid for GameState {
  fn player_pos(&self) -> Pos {
    self.player_pos
  }
}

impl TileGet<usize> for GameState {
  fn get(&self, index: usize) -> Option<Tile> {
    if index < self.mask.len() {
      Some(self.get_unchecked(index))
    } else {
      None
    }
  }
  fn get_unchecked(&self, index: usize) -> Tile {
    if index == self.pos_to_index_unchecked(self.player_pos) {
      debug_assert!(self.get_fake_unchecked(index) == FakeTile::EMTPY);
      Tile::Player
    } else {
      Tile::from(self.get_fake_unchecked(index))
    }
  }
}

impl TileGet<Pos> for GameState {
  fn get(&self, pos: Pos) -> Option<Tile> {
    let index = self.pos_to_index_unchecked(pos);
    self.get(index)
  }
  fn get_unchecked(&self, pos: Pos) -> Tile {
    if pos == self.player_pos {
      debug_assert!(self.get_fake_unchecked(self.pos_to_index_unchecked(pos)) == FakeTile::EMTPY);
      Tile::Player
    } else {
      let index = self.pos_to_index_unchecked(pos);
      Tile::from(self.get_fake_unchecked(index))
    }
  }
}

impl Playable for GameState {
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, PlayableError> {
    let mut current_pos = self.player_pos + dir;
    // check if position was valid - is the same as calling dir.valid() obviously
    if current_pos == self.player_pos {
      return Err(PlayableError::InvalidDirection);
    }

    let starting_index = self
      .pos_to_index(current_pos)
      .ok_or(PlayableError::BadMove)?;
    // The first tile the player moves to
    // Tells us how many tiles to move
    let starting_tile = self.get_fake_unchecked(starting_index);
    if starting_tile == FakeTile::EMTPY {
      return Err(PlayableError::BadMove);
    }
    let move_amount = starting_tile.amount();

    let mut moves = Vec::with_capacity(move_amount.into());
    // push the tile that gave us the amount
    moves.push(starting_index);
    // collect positions and check for collision -> BadMove
    for _ in 1..move_amount {
      current_pos += dir;
      let index = self
        .pos_to_index(current_pos)
        .ok_or(PlayableError::BadMove)?;
      let tile = self.get_fake_unchecked(index);
      if tile == FakeTile::EMTPY {
        return Err(PlayableError::BadMove);
      }
      moves.push(index)
    }

    // return movements
    Ok(moves)
  }

  /// TODO: give the user a more efficient way to execute a move checked by check_move?
  /// Maybe introduce a CheckedMove type to do that safely?
  ///
  /// For the return see check_move function.
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, PlayableError> {
    // commit movements
    let moves = self.check_move(dir)?;

    let mut iter = moves.iter().rev();
    // If check_move is successful the returned vec contains at least one element
    let &player_index = iter.next().unwrap();
    self.player_pos = self.index_to_pos_unchecked(player_index);

    self.mask.set(player_index, false);

    for &index in iter {
      self.mask.set(index, false)
    }

    // update the moves array
    self
      .moves
      .push((dir, Amount::new_unchecked(moves.len() as u8)));
    Ok(moves)
  }

  fn undo_move(&mut self) -> Result<(), PlayableError> {
    let last_move = self.moves.last().ok_or(PlayableError::BadMove)?;
    let &(dir, amount) = last_move;
    // dir.valid()?; // always valid we control the moves
    let dir = !dir; // invert the direction bit flag magic

    let end_pos = self.player_pos + dir * amount.amount();

    if !self.is_valid_pos(end_pos) {
      // Restore the moves array back to a "valid" state
      return Err(PlayableError::UndoInvalidMove);
    }
    // 1..amount because we don't want to uncheck the player
    for already_moved_tiles in 0..amount.amount() {
      // pos_to_index_unchecked is safe her because the initial player position is considered safe
      //  and we verified that end_pos is safe. Therefore all positions in between must also be safe.

      let index = self.pos_to_index_unchecked(self.player_pos);

      if self.player_pos != end_pos && self.mask[index] {
        self.move_set(self.player_pos, !dir, already_moved_tiles, false); // undo the partial undo in order to revert the breakage of the mask.

        return Err(PlayableError::UndoInvalidMove);
      }

      self.mask.set(index, true);
      self.player_pos += dir;
    }
    // move the player pos without setting the mask to true

    let _ = self.moves.pop();

    Ok(())
  }

  fn game_field(&self) -> &GameField {
    &self.game_field
  }

  fn move_count(&self) -> usize {
    self.moves().len()
  }
}

impl Grid2D for GameState {
  fn dimensions(&self) -> Size2D {
    self.game_field.dimensions()
  }

  fn tile_count(&self) -> usize {
    self.mask.len()
  }

  // The following functions are implemented as wrappers to make sure they aren't generated again
  fn is_valid_pos(&self, pos: Pos) -> bool {
    self.game_field.is_valid_pos(pos)
  }

  fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    self.game_field.valid_pos(pos)
  }

  fn valid_index(&self, index: usize) -> Option<usize> {
    self.game_field.valid_index(index)
  }

  fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    self.game_field.pos_to_index(pos)
  }

  fn pos_to_index_unchecked(&self, pos: Pos) -> usize {
    self.game_field.pos_to_index_unchecked(pos)
  }

  fn index_to_pos(&self, index: usize) -> Option<Pos> {
    self.game_field.index_to_pos(index)
  }

  fn index_to_pos_unchecked(&self, index: usize) -> Pos {
    self.game_field.index_to_pos_unchecked(index)
  }
}

impl Display for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.display_fmt(f)
  }
}
impl Debug for GameState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "MASK: ")?;
    let Size2D { x_size, y_size } = self.dimensions();
    for y in 0..y_size {
      for x in 0..x_size {
        let pos = Pos::new(x as isize, y as isize);
        let index = self.pos_to_index_unchecked(pos);

        write!(f, "{}", self.mask[index] as u8)?;
      }
      writeln!(f)?;
    }

    writeln!(f, "Field: ")?;
    write!(f, "{}", self.game_field())
  }
}

impl From<&GameState> for String {
  fn from(game_field: &GameState) -> Self {
    game_field.to_string()
  }
}
