pub use super::*;

#[test]
fn greed_from_str() {
  //assert_eq!(
  //  *Greed::try_from("1").unwrap().field(),
  //  GameField {
  //    vec: Vec::from([Tile::V1]),
  //    x_size: 1,
  //    y_size: 0
  //  }
  //)
}

mod seed_test {
  pub use super::*;

  const TEST_TILE_PROBS: [u8; 9] = [17, 34, 51, 68, 85, 102, 119, 136, 153];

  #[test]
  fn test_parsing_a_seed() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#112233445566778899").unwrap(),
      Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        Some(TileProbs::from(TEST_TILE_PROBS))
      )
    )
  }
  #[test]
  fn test_parsing_a_seed_no_tile_probs() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9").unwrap(),
      Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        None
      )
    )
  }
  #[test]
  fn test_serializing_a_seed() {
    assert_eq!(
      &String::from(&Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        Some(TileProbs::from(TEST_TILE_PROBS))
      )),
      "ABCD_abcd_1234#6x9#112233445566778899",
    )
  }
  #[test]
  fn test_serializing_a_seed_no_tile_probs() {
    assert_eq!(
      &String::from(&Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        None
      )),
      "ABCD_abcd_1234#6x9",
    )
  }
}
