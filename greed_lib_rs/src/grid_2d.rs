use super::{Pos, Size2D};

pub trait Grid2D {
  /// For the default implementations to work the each
  /// value in the returned tuple must not exceed isize::MAX.
  fn dimensions(&self) -> Size2D;
  /// Can also be interpreted as the maximum score
  fn tile_count(&self) -> usize {
    let Size2D { x_size, y_size } = self.dimensions();
    x_size * y_size
  }
  fn is_valid_pos(&self, pos: Pos) -> bool {
    let Size2D { x_size, y_size } = self.dimensions();
    let x_size = x_size as isize;
    let y_size = y_size as isize;
    (0..x_size).contains(&pos.x) && (0..y_size).contains(&pos.y)
  }
  fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    if self.is_valid_pos(pos) {
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
    let x_size = self.dimensions().x_size;
    pos.x as usize + (pos.y as usize) * x_size
  }
  fn index_to_pos(&self, index: usize) -> Option<Pos> {
    let index = self.valid_index(index)?;
    Some(self.index_to_pos_unchecked(index))
  }
  /// Assumes that the index is valid
  fn index_to_pos_unchecked(&self, index: usize) -> Pos {
    let x_size = self.dimensions().x_size;
    let y = (index / x_size) as isize;
    let x = (index % x_size) as isize;
    // let x = (x_size * y_size - y * x_size) as isize;
    Pos::new(x, y)
  }
}
