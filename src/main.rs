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
  let game = greed::Greed::try_from("1034@\n17827\n").unwrap();
  println!("FIELD:\n{}", game.field());
}
