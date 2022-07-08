use super::*;
use bitvec::prelude as bv;
use std::rc::Rc;

pub type Amount = u8;

/// This mutable structure represents a modified game field.
/// It encodes which fields have been consumed and the player pos.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameState {
  /// If a tile has a false mask it should be considered as an empty tile.
  /// The player also has a false mask.
  mask: bv::BitVec,
  player_pos: Pos,
  moves: Vec<(Direction, Amount)>,
  game_field: Rc<GameField>,
}

impl GameState {
  pub fn new(game_field: Rc<GameField>) -> Self {
    let player_pos = game_field.player_pos();
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

  pub fn game_field(&self) -> &GameField {
    &self.game_field
  }

  fn get_fake_unchecked(&self, index: usize) -> FakeTile {
    if self.mask[index] == false {
      FakeTile::EMTPY
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
    self.player_pos = self.index_to_pos_unchecked(player_index);
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

impl TileGrid for GameState {
  fn dimensions(&self) -> (usize, usize) {
    self.game_field.dimensions()
  }

  fn player_pos(&self) -> Pos {
    self.player_pos
  }
}

impl TileGet<usize> for GameState {
  fn get(&self, index: usize) -> Option<Tile> {
    todo!()
  }
  fn get_unchecked(&self, index: usize) -> Tile {
    todo!()
  }
}

impl TileGet<Pos> for GameState {
  fn get(&self, pos: Pos) -> Option<Tile> {
    todo!()
  }
  fn get_unchecked(&self, pos: Pos) -> Tile {
    todo!()
  }
}
