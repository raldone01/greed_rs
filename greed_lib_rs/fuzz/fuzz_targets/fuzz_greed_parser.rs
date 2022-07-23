#![no_main]
use libfuzzer_sys::fuzz_target;

use greed_lib_rs::{Direction, Greed, Playable};

fuzz_target!(|data: &str| {
  if let Ok(mut game) = Greed::load_from_string(data) {
    let _ = game.move_(Direction::RIGHT);
    let _ = game.undo_move();
    let _ = game.undo_move();
  }
});
