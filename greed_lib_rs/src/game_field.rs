use super::{
  FakeTile, FakeTileConversionError, GameFieldParserError, GameState, Grid2D, Pos, Seed, Size2D,
  Tile, TileChooser, TileGet, TileGrid, DEFAULT_SIZE,
};
use rand::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha512};
use std::fmt::{Debug, Display};

/// This immutable structure represents the initial state of a game of greed.
/// It contains all tiles including the player.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct GameField {
  /// The fake tile the player is on MUST be an EMPTY tile.
  pub(super) vec: Box<[FakeTile]>,
  size: Size2D,
  /// initial player_pos
  player_pos: Pos,
}

impl GameField {
  /// Not exposed because it is counter intuitive
  pub(super) fn new_from_game_state(game_state: &GameState) -> Self {
    let vec = (0..game_state.tile_count())
      .map(|index| game_state.get_fake_unchecked(index))
      .collect();

    Self {
      vec,
      size: game_state.dimensions(),
      player_pos: game_state.player_pos(),
    }
  }

  fn new_random(tile_chooser: &mut TileChooser<impl Rng>, size: Size2D) -> Self {
    let mut vec: Box<_> = (0..size.tile_count())
      .map(|_| tile_chooser.choose())
      .collect();

    let player_pos = Pos {
      x: tile_chooser.rng.gen_range(0..size.x_size) as isize,
      y: tile_chooser.rng.gen_range(0..size.y_size) as isize,
    };
    vec[size.pos_to_index_unchecked(player_pos)] = FakeTile::EMTPY;

    Self {
      vec,
      size,
      player_pos,
    }
  }

  pub fn from_seed(seed: &Seed) -> GameField {
    let mut hasher = Sha512::new();
    hasher.update(seed.user_str());
    let hash = hasher.finalize();
    let used_hash = <[u8; 16]>::try_from(&hash[0..16])
      .expect("Can never fail since we actually statically know the size");
    // init the random gen with the first 16 bytes of the hash
    let mut rng = rand_pcg::Pcg64Mcg::from_seed(used_hash);
    let mut tile_chooser = TileChooser::new(&mut rng, seed.tile_probabilities());
    GameField::new_random(&mut tile_chooser, seed.size())
  }
}

impl TileGrid for GameField {
  fn player_pos(&self) -> Pos {
    self.player_pos
  }
}

impl Grid2D for GameField {
  fn dimensions(&self) -> Size2D {
    self.size
  }

  fn tile_count(&self) -> usize {
    self.vec.len()
  }

  // The following functions are implemented as wrappers to make sure they aren't generated again
  fn is_valid_pos(&self, pos: Pos) -> bool {
    self.size.is_valid_pos(pos)
  }

  fn valid_pos(&self, pos: Pos) -> Option<Pos> {
    self.size.valid_pos(pos)
  }

  fn valid_index(&self, index: usize) -> Option<usize> {
    self.size.valid_index(index)
  }

  fn pos_to_index(&self, pos: Pos) -> Option<usize> {
    self.size.pos_to_index(pos)
  }

  fn pos_to_index_unchecked(&self, pos: Pos) -> usize {
    self.size.pos_to_index_unchecked(pos)
  }

  fn index_to_pos(&self, index: usize) -> Option<Pos> {
    self.size.index_to_pos(index)
  }

  fn index_to_pos_unchecked(&self, index: usize) -> Pos {
    self.size.index_to_pos_unchecked(index)
  }
}

impl From<&Seed> for GameField {
  fn from(seed: &Seed) -> Self {
    GameField::from_seed(seed)
  }
}

impl From<&GameField> for String {
  fn from(game_field: &GameField) -> Self {
    game_field.to_string()
  }
}

impl TryFrom<&str> for GameField {
  type Error = GameFieldParserError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let default_size = DEFAULT_SIZE;
    let mut vec = Vec::with_capacity(default_size.tile_count());

    let mut x_size = None;
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut player_pos = None;
    for c in value.chars() {
      match c {
        '\n' => {
          if x_pos == 0 {
            return Err(GameFieldParserError::EmptyLine);
          }
          if let Some(x_size) = x_size {
            if x_pos != x_size {
              return Err(GameFieldParserError::NotRectangular);
            }
          } else {
            x_size = Some(x_pos);
          }
          x_pos = 0;
          y_pos += 1;
        },
        c => {
          // This technically doesn't need to be always calculated, but its po
          let pos = {
            Pos::new(
              x_pos
                .try_into()
                .map_err(|_| GameFieldParserError::InvalidSize)?,
              y_pos
                .try_into()
                .map_err(|_| GameFieldParserError::InvalidSize)?,
            )
          };
          let tile = Tile::try_from(c).map_err(|err| GameFieldParserError::InvalidCharacter {
            found: err.found,
            pos,
          })?;

          match FakeTile::try_from(tile) {
            Ok(tile) => vec.push(tile),
            Err(FakeTileConversionError::PlayerTile) => {
              if let Some(first) = player_pos {
                return Err(GameFieldParserError::AmbiguousPlayer { first, second: pos });
              }
              player_pos = Some(pos);
              vec.push(FakeTile::EMTPY);
            },
          }

          x_pos += 1;
        },
      }
    }

    if y_pos == 0 {
      return Err(GameFieldParserError::InvalidSize);
    }

    if x_pos != 0 {
      return Err(GameFieldParserError::NoTrailingNewLine);
    }

    let size = Size2D::new_unchecked(
      x_size.expect("since x_size is set on newlines and y_pos != 0"),
      y_pos,
    );
    assert!(vec.len() == size.tile_count());
    let vec = vec.into_boxed_slice();

    let game_field = GameField {
      vec,
      size,
      player_pos: player_pos.ok_or(GameFieldParserError::PlayerNotFound)?,
    };
    Ok(game_field)
  }
}

impl TileGet<usize> for GameField {
  fn get(&self, index: usize) -> Option<Tile> {
    // player_pos is always valid(Hopefully)
    if index == self.pos_to_index_unchecked(self.player_pos) {
      Some(Tile::Player)
    } else {
      // Never masked since we are GF

      Some(Tile::from(*self.vec.get(index)?))
    }
  }
  fn get_unchecked(&self, index: usize) -> Tile {
    // player_pos is always valid(Hopefully)
    if index == self.pos_to_index_unchecked(self.player_pos) {
      Tile::Player
    } else {
      // Never masked since we are GF

      Tile::from(self.vec[index])
    }
  }
}

impl TileGet<Pos> for GameField {
  fn get(&self, pos: Pos) -> Option<Tile> {
    if pos == self.player_pos {
      Some(Tile::Player)
    } else {
      // Never masked since we are GF

      let index = self.pos_to_index(pos)?;
      Some(Tile::from(*self.vec.get(index)?))
    }
  }
  fn get_unchecked(&self, pos: Pos) -> Tile {
    if pos == self.player_pos {
      Tile::Player
    } else {
      // Never masked since we are GF

      let index = self.pos_to_index_unchecked(pos);
      Tile::from(self.vec[index])
    }
  }
}

impl Display for GameField {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.display_fmt(f)
  }
}

impl<'de> Deserialize<'de> for GameField {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    // Sadly serde does not provide a validation hook
    #[derive(Deserialize)]
    struct InnerGameField {
      vec: Box<[FakeTile]>,
      size: Size2D,
      player_pos: Pos,
    }
    let InnerGameField {
      vec,
      size,
      player_pos,
    } = InnerGameField::deserialize(deserializer)?;
    // validate that the player pos is valid
    if !size.is_valid_pos(player_pos) {
      let Size2D { x_size, y_size } = size;
      Err(de::Error::custom(format!(
        "Player pos {} is not valid. Expected (x: 0..{x_size}, y: 0..{y_size})",
        player_pos
      )))?
    }
    // validate that players underlying tile is an EMPTY tile
    let tile = vec[size.pos_to_index_unchecked(player_pos)];
    if tile != FakeTile::EMTPY {
      Err(de::Error::custom(format!(
        "Tile at player pos {} not empty tile instead it is {}",
        player_pos,
        Tile::from(tile),
      )))?
    }
    Ok(Self {
      vec,
      size,
      player_pos,
    })
  }
}
