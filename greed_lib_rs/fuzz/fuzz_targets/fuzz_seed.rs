#![no_main]
use libfuzzer_sys::fuzz_target;

use greed_lib_rs::{Seed, Size2D};
use std::convert::TryFrom;

fuzz_target!(|data: &str| {
  if let Ok(seed) = Seed::try_from(data) {
    let seed = String::from(seed);

    let mut split_data = data.split('#');
    let mut split_seed = seed.split('#');
    let user_str_data = split_data.next().unwrap();
    let user_str_seed = split_seed.next().unwrap();
    assert_eq!(user_str_data, user_str_seed);

    let size_data = split_data
      .next()
      .map_or(Ok(Size2D::DEFAULT_SIZE), Size2D::try_from)
      .unwrap();
    let size_seed = split_seed.next().map(Size2D::try_from).unwrap().unwrap();
    assert_eq!(size_data, size_seed);

    let tile_probs_data = split_data.next().map(str::to_lowercase);
    let tile_probs_seed = split_seed.next().map(str::to_lowercase);
    assert_eq!(tile_probs_data, tile_probs_seed);

    assert!(split_data.next().is_none());
    assert!(split_seed.next().is_none());
  }
});
