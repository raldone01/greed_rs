use super::{Grid2D, Pos, Size2D, Tile};
use std::{fmt::Write, iter::FusedIterator};

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

pub trait TileGrid: TileGet<usize> + TileGet<Pos> + Grid2D {
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

  fn display_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in self.rows() {
      for tile in row {
        f.write_char(char::from(tile))?;
      }
      f.write_char('\n')?;
    }
    Ok(())
  }

  /// All types that implement TileGrid should also implement Display
  /// so you can alternatively call to_string which will usually end up calling this function.
  /// This name is a bit weird to avoid colliding with the ToString trait.
  fn to_string_tile_grid(&self) -> String {
    // Don't forget about the new line characters
    let mut out = String::with_capacity(self.tile_count() + self.dimensions().y_size);
    for row in self.rows() {
      for tile in row {
        out.push(char::from(tile));
      }
      out.push('\n');
    }
    out
  }

  fn player_pos(&self) -> Pos;
}
