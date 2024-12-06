#![allow(dead_code)]

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;
#[cfg(feature = "termion")]
use crate::termion::run;

use greed_lib_rs::{Direction, Greed, Playable};

#[allow(unreachable_code, unused_variables)]
fn main() {
  let gf = Greed::load_from_string("01336\n11117\n").unwrap_err();
  println!("{gf}");
  return;

  let mut greed = Greed::load_from_string("0133@\n11117\n").unwrap();

  println!("Initial field:\n{}", greed.game_state());

  greed.move_(Direction::LEFT).unwrap();
  println!("move_left:\n{}", greed.game_state());
  greed.move_(Direction::DOWN).unwrap();
  println!("move_down:\n{}", greed.game_state());
  greed.move_(Direction::RIGHT).unwrap();
  println!("move_right:\n{}", greed.game_state());

  greed.undo_move().unwrap();
  println!("undo_move:\n{}", greed.game_state());
  greed.undo_move().unwrap();
  println!("undo_move:\n{}", greed.game_state());
  greed.undo_move().unwrap();
  println!("undo_move:\n{}", greed.game_state());
  println!("Final State:\n{:?}", greed.game_state());

  println!("NAME: {}", greed.name());

  greed.move_(Direction::LEFT).unwrap();
  println!("move_left:\n{}", greed.game_state());

  let save_file = greed.save_to_string();
  println!("Save File:\n{}", save_file);
  let parsed_greed = Greed::load_from_string(&save_file).unwrap();
  let new_save_file = parsed_greed.save_to_string();
  println!("New save File:\n{}", new_save_file);
  assert_eq!(save_file, new_save_file);
}
