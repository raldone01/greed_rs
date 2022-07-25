use alloc::vec::Vec;

use super::{Direction, GameField, PlayableError};

pub trait Playable {
  /// Checks if a move would be valid.
  /// Returns the indices that would be consumed excluding the old player pos, but including the new player pos.
  /// They are in order from the closest to the farthest.
  /// So `ret.unwrap().len()` would be the amount of tiles consumed.
  /// # Errors
  /// * If `dir` is invalid.
  /// * A move in `dir` is imposible.
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, PlayableError>;
  /// Returns the positions that were consumed like `check_move`.
  /// # Errors
  /// see `check_move`
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, PlayableError>;

  /// Undoes a previously executed Move
  /// # Errors
  /// * If no moves are left to undo.
  /// * If not enough data is left to undo the move
  fn undo_move(&mut self) -> Result<(), PlayableError>;
  fn game_field(&self) -> &GameField;
  fn move_count(&self) -> usize;
  fn is_game_complete(&self) -> bool {
    for &dir in Direction::all_directions_cw() {
      if self.check_move(dir) != Err(PlayableError::BadMove) {
        return false;
      }
    }
    true
  }
  fn game_complete(&self) -> Option<&Self> {
    if self.is_game_complete() {
      Some(self)
    } else {
      None
    }
  }
}
