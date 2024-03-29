mod utils;

use std::convert::TryFrom;

use greed_lib_rs::{Direction, Greed, GreedBuilder, Playable, Seed, Size2D, UserString};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Game {
  greed: Greed,
}
#[wasm_bindgen]
pub fn set_panic_hook() {
  utils::set_panic_hook();
}

#[wasm_bindgen]
impl Game {
  pub fn from_string(value: &str) -> Result<Game, String> {
    Ok(Self {
      greed: Greed::load_from_string(value).map_err(|err| format!("{err}"))?,
    })
  }
  pub fn generate(x_size: usize, y_size: usize, seed: &str) -> Result<Game, String> {
    let size = Size2D::new(x_size, y_size).map_err(|err| format!("{err:?}"))?;
    let seed = if seed.is_empty() {
      Seed::new_random(size, None)
    } else {
      let seed = UserString::try_from(seed).map_err(|err| format!("{err:?}"))?;
      Seed::new(seed, size, None)
    };
    Ok(Self {
      greed: GreedBuilder::new().seed(seed).build(),
    })
  }
  pub fn print(&self) -> String {
    format!("{}", self.greed.game_state())
  }
  pub fn seed(&self) -> String {
    self
      .greed
      .seed()
      .map_or_else(|| "No Seed".into(), String::from)
  }
  pub fn move_numpad(&mut self, key: u8) -> Result<(), String> {
    let dir = match key {
      1 => Direction::DOWN.union(Direction::LEFT),
      2 => Direction::DOWN,
      3 => Direction::DOWN.union(Direction::RIGHT),
      4 => Direction::LEFT,
      6 => Direction::RIGHT,
      7 => Direction::UP.union(Direction::LEFT),
      8 => Direction::UP,
      9 => Direction::UP.union(Direction::RIGHT),
      _ => return Ok(()), // Fall through for cases like 0 and 5
    };
    self
      .greed
      .move_(dir)
      .map(|_| ())
      .map_err(|err| err.to_string())
  }

  pub fn undo(&mut self) -> Result<(), String> {
    self
      .greed
      .undo_move()
      .map(|_| ())
      .map_err(|err| err.to_string())
  }
  pub fn is_stuck(&mut self) -> bool {
    Direction::ALL_DIRECTIONS_CW
      .iter()
      .all(|&dir| self.greed.check_move(dir).is_err())
  }
  pub fn save(&self) -> String {
    self.greed.save_to_string()
  }
}
