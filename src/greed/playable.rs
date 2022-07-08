use super::*;

pub trait Playable {
  fn check_move(&self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  fn move_(&mut self, dir: Direction) -> Result<Vec<usize>, GreedError>;
  fn undo_move(&mut self) -> Result<(), GreedError>;
  fn game_field(&self) -> &GameField;
}
