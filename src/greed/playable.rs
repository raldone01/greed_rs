use super::*;

pub trait Playable {
  /// Returns the positions that would be consumed.
  /// They are in order from the closest to the farthest.
  /// # Examples
  /// ```rust
  /// use greed::*;
  ///
  /// let game = Greed::new(...);
  /// let move_score = game.check_move(dir).len();
  /// ```
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  /// Returns the positions that were consumed.
  /// They are in order from the closest to the farthest.
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  fn undo_move(&mut self) -> Result<(), GreedError>;
  fn game_field(&self) -> &GameField;
  fn move_count(&self) -> usize;
}
