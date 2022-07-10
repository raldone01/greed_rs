use super::*;

pub trait Playable {
  /// Checks if a move would be valid.
  /// Returns the indices that would be consumed excluding the old player pos, but including the new player pos.
  /// They are in order from the closest to the farthest.
  /// So `ret.unwrap().len()` would be the amount of tiles consumed.
  /// # Examples
  /// ```rust
  /// use greed::*;
  ///
  /// let game = Greed::new(...);
  /// let move_score = game.check_move(dir).len();
  /// ```
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  /// Returns the positions that were consumed like `check_move`.
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  fn undo_move(&mut self) -> Result<(), GreedError>;
  fn game_field(&self) -> &GameField;
  fn move_count(&self) -> usize;
}
