#![no_main]
use std::rc::Rc;

use libfuzzer_sys::fuzz_target;

use greed_lib_rs::{Direction, GameField, GameState, Playable, Seed};

fuzz_target!(|data: (Seed, [Direction; 64])| {
  let (seed, dirs) = data;

  let game_field = Rc::new(GameField::from_seed(&seed));
  let mut game_state = GameState::new(game_field);
  for dir in dirs {
    let _ = game_state.move_(dir);
  }
});
