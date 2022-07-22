#![no_main]
use libfuzzer_sys::fuzz_target;

use greed_lib_rs::Greed;

fuzz_target!(|data: &[u8]| {
  if let Ok(data) = std::str::from_utf8(data) {
    let _ = Greed::load_from_string(data);
  }
});
