#![allow(dead_code)]

mod greed;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

mod ui;

use std::rc::Rc;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;
use crate::greed::{GameState, Playable};
#[cfg(feature = "termion")]
use crate::termion::run;

fn main() {
  // let game = greed::Greed::try_from("1034@\n17817\n").unwrap();
  // println!("FIELD:\n{}", game.field());
  #[allow(unused_mut)]
  let mut field = Rc::new(greed::GameField::try_from("0133@\n11117\n").unwrap());
  println!("FIELD:\n{}", field);

  let mut state = GameState::new(field);

  state.move_(greed::Direction::LEFT).unwrap();
  println!("move_left:\n{}", state);
  state.move_(greed::Direction::DOWN).unwrap();
  println!("move_down:\n{}", state);
  state.move_(greed::Direction::RIGHT).unwrap();
  println!("move_right:\n{}", state);

  state.undo_move().unwrap();
  println!("undo_move:\n{}", state);
  state.undo_move().unwrap();
  println!("undo_move:\n{}", state);
  state.undo_move().unwrap();
  println!("undo_move:\n{}", state);
  println!("Final State\n{:?}", state);
}
