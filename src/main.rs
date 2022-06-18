mod greed;

/* #[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

mod ui;

#[cfg(feature = "crossterm")]
use crate::crossterm::run;
#[cfg(feature = "termion")]
use crate::termion::run; */

fn main() {
  let tile = greed::Tile::Player;
  println!("Hello, world!");
}
