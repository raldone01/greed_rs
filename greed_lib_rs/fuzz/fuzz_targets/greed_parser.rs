#![no_main]
use libfuzzer_sys::fuzz_target;

use greed_lib_rs::{Greed, Seed};
use rand::prelude::*;

fuzz_target!(|data: &[u8]| {
  /* let mut hasher = Sha512::new();
  hasher.update(seed.user_str());
  let hash = hasher.finalize();
  let used_hash = <[u8; 16]>::try_from(&hash[0..16]).unwrap();
  let mut rng = rand_pcg::Pcg64Mcg::from_seed(data);

  let seed = Seed::;
  // create random game_field


  // run n random moves on the game_field
  let rng = rand::
  if let Ok(data) = std::str::from_utf8(data) {
    let _ = Greed::load_from_string(data);
  } */
  todo!();
});
