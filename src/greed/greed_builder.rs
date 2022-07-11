use super::{GameField, Greed, Seed, Size2D, TileProbs};

pub struct GreedBuilder {
  name: Option<String>,
}

impl GreedBuilder {
  pub fn new() -> Self {
    Self { name: None }
  }

  pub fn size(self, size: Size2D) -> SizeProbGreedBuilder {
    SizeProbGreedBuilder {
      name: self.name,
      size: Some(size),
      tile_probs: None,
    }
  }
  pub fn tile_probs(self, probs: TileProbs) -> SizeProbGreedBuilder {
    SizeProbGreedBuilder {
      name: self.name,
      size: None,
      tile_probs: Some(probs),
    }
  }

  pub fn seed(self, seed: Seed) -> SeedGreedBuilder {
    SeedGreedBuilder {
      seed,
      name: self.name,
    }
  }

  pub fn name(&mut self, name: String) -> &mut Self {
    self.name = Some(name);
    self
  }

  pub fn build(self) -> Greed {
    SizeProbGreedBuilder {
      name: self.name,
      size: None,
      tile_probs: None,
    }
    .build()
  }
}
pub struct SeedGreedBuilder {
  name: Option<String>,
  seed: Seed,
}
impl SeedGreedBuilder {
  pub fn name(&mut self, name: String) -> &mut Self {
    self.name = Some(name);
    self
  }
  pub fn build(self) -> Greed {
    let name = self
      .name
      .unwrap_or_else(|| String::from(self.seed.user_str()));
    Greed::new_from_builder(name, self.seed)
  }
}

pub struct SizeProbGreedBuilder {
  name: Option<String>,
  size: Option<Size2D>,
  tile_probs: Option<TileProbs>,
}
impl SizeProbGreedBuilder {
  pub fn build(self) -> Greed {
    let size = self
      .size
      .unwrap_or_else(GameField::default_classic_game_dimensions);
    let Size2D { x_size, y_size, .. } = size;
    if x_size < 1 || y_size < 1 || x_size > isize::MAX as usize || y_size > isize::MAX as usize {
      // TODO: maybe add this check to seed somewhere
      // TODO Fix
      //return Err(GameFieldParserError::InvalidSize);
    }

    let seed = Seed::new_random(size, self.tile_probs);
    SeedGreedBuilder {
      name: self.name,
      seed,
    }
    .build()
  }
  pub fn size(&mut self, size: Size2D) -> &mut Self {
    self.size = Some(size);
    self
  }
  pub fn tile_probs(&mut self, probs: TileProbs) -> &mut Self {
    self.tile_probs = Some(probs);
    self
  }
  pub fn name(&mut self, name: String) -> &mut Self {
    self.name = Some(name);
    self
  }
}
