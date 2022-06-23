#![allow(unused_variables, dead_code)]

mod greed;

#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

mod ui;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;
#[cfg(feature = "termion")]
use crate::termion::run;

fn main() {
  // let game = greed::Greed::try_from("1034@\n17817\n").unwrap();
  // println!("FIELD:\n{}", game.field());

  let mut field = greed::GameField::try_from("1034@\n17817\n").unwrap();
  println!("FIELD:\n{}", field);

  field
    .move_(greed::Direction::DOWN | greed::Direction::LEFT)
    .unwrap();
  println!("FIELD:\n{}", field);
}
