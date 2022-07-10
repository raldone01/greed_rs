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
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, PlayableError>;
  /// Returns the positions that were consumed.
  /// They are in order from the closest to the farthest.
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, PlayableError>;
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
