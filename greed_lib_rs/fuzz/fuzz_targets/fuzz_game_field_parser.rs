#![no_main]
use std::{convert::TryFrom, sync::Arc};

use libfuzzer_sys::fuzz_target;

use greed_lib_rs::{Direction, GameField, GameState, Playable};

fuzz_target!(|data: &str| {
  //if data.len() <= 0 || data.chars().position(|c| c == '\n') == Some(data.len() - 1) {
  //  return;
  //}
  if let Ok(game) = GameField::try_from(data) {
    let mut game = GameState::new(Arc::new(game));
    let _ = game.move_(Direction::RIGHT);
    let _ = game.undo_move();
    let _ = game.undo_move();
  }
});
