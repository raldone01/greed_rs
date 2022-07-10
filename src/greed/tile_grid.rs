use std::iter::FusedIterator;

use super::*;

pub trait TileGet<I> {
  fn get(&self, index: I) -> Option<Tile>;
  fn get_unchecked(&self, index: I) -> Tile;
}

pub struct StrideTileIterator<'a, T: TileGrid + ?Sized> {
  start: usize,
  stride: usize,
  end: usize,
  grid: &'a T,
}
impl<'a, T: TileGrid + ?Sized> Iterator for StrideTileIterator<'a, T> {
  type Item = Tile;

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.start;
    if index < self.end {
      self.start = index + self.stride;
      Some(self.grid.get_unchecked(index))
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = (self.end - self.start) / self.stride;
    (remaining, Some(remaining))
  }
}
impl<'a, T: TileGrid + ?Sized> DoubleEndedIterator for StrideTileIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    if self.start < self.end - self.stride {
      self.end -= self.stride;
      Some(self.grid.get_unchecked(self.end))
    } else {
      None
    }
  }
}

impl<'a, T: TileGrid + ?Sized> FusedIterator for StrideTileIterator<'a, T> {}
impl<'a, T: TileGrid + ?Sized> ExactSizeIterator for StrideTileIterator<'a, T> {}

pub struct ColIterator<'a, T: TileGrid + ?Sized> {
  start_col: usize,
  end_col: usize,
  grid: &'a T,
}
impl<'a, T: TileGrid + ?Sized> Iterator for ColIterator<'a, T> {
  type Item = StrideTileIterator<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    let Size2D { x_size, y_size } = self.grid.dimensions();
    let col = self.start_col;
    if col < self.end_col {
      self.start_col = col + 1;
      Some(StrideTileIterator {
        start: col,
        stride: x_size,
        end: col + 1 + (y_size - 1) * x_size, // y_size * x_size
        grid: self.grid,
      })
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.end_col - self.start_col;
    (remaining, Some(remaining))
  }
}
impl<'a, T: TileGrid + ?Sized> DoubleEndedIterator for ColIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let Size2D { x_size, y_size } = self.grid.dimensions();
    if self.start_col < self.end_col {
      self.end_col -= 1;
      Some(StrideTileIterator {
        start: self.end_col,
        stride: x_size,
        end: self.end_col + 1 + (y_size - 1) * x_size, // y_size * x_size
        grid: self.grid,
      })
    } else {
      None
    }
  }
}

impl<'a, T: TileGrid + ?Sized> FusedIterator for ColIterator<'a, T> {}
impl<'a, T: TileGrid + ?Sized> ExactSizeIterator for ColIterator<'a, T> {}

pub struct RowIterator<'a, T: TileGrid + ?Sized> {
  offset: usize,
  end: usize,
  grid: &'a T,
}
impl<'a, T: TileGrid + ?Sized> Iterator for RowIterator<'a, T> {
  type Item = StrideTileIterator<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    let x_size = self.grid.dimensions().x_size;
    let offset = self.offset;
    if offset < self.end {
      self.offset = offset + x_size;
      Some(StrideTileIterator {
        start: offset,
        stride: 1,
        end: offset + x_size,
        grid: self.grid,
      })
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let x_size = self.grid.dimensions().x_size;
    let remaining = (self.end - self.offset) / x_size;
    (remaining, Some(remaining))
  }
}
impl<'a, T: TileGrid + ?Sized> DoubleEndedIterator for RowIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    let x_size = self.grid.dimensions().x_size;
    if self.offset < self.end - x_size {
      self.end -= x_size;
      Some(StrideTileIterator {
        start: self.end,
        stride: 1,
        end: self.end + x_size,
        grid: self.grid,
      })
    } else {
      None
    }
  }
}

impl<'a, T: TileGrid + ?Sized> FusedIterator for RowIterator<'a, T> {}
impl<'a, T: TileGrid + ?Sized> ExactSizeIterator for RowIterator<'a, T> {}

pub struct TileIterator<'a, T: TileGrid + ?Sized> {
  start: usize,
  end: usize,
  grid: &'a T,
}

impl<'a, T: TileGrid + ?Sized> Iterator for TileIterator<'a, T> {
  type Item = Tile;

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.start;
    if index < self.end {
      self.start = index + 1;
      Some(self.grid.get_unchecked(index))
    } else {
      None
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = self.end - self.start;
    (remaining, Some(remaining))
  }
}
impl<'a, T: TileGrid + ?Sized> DoubleEndedIterator for TileIterator<'a, T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    if self.start < self.end {
      self.end -= 1;
      Some(self.grid.get_unchecked(self.end))
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

  fn iter(&self) -> TileIterator<Self> {
    TileIterator {
      start: 0,
      end: self.tile_count(),
      grid: self,
    }
  }

  fn cols(&self) -> ColIterator<Self> {
    let x_size = self.dimensions().x_size;
    ColIterator {
      start_col: 0,
      end_col: x_size,
      grid: self,
    }
  }

  fn rows(&self) -> RowIterator<Self> {
    RowIterator {
      offset: 0,
      end: self.tile_count(),
      grid: self,
    }
  }

  fn score(&self) -> usize {
    self.iter().fold(
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

  fn player_pos(&self) -> Pos;
}
