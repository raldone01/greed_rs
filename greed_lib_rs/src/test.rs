pub use super::*;

mod seed_test {
  pub use super::*;

  #[test]
  fn test_seed_to_many() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#112233445566778899#1212312"),
      Err(SeedConversionError::UnexpectedHashTag)
    )
  }

  #[test]
  fn test_seed_no_size() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234"),
      Ok(Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        DEFAULT_SIZE,
        Some(DEFAULT_TILE_PROBABILITIES) // Could also use None
      ))
    )
  }
  #[test]
  fn test_seed_only_probs() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#112233445566778899"),
      Err(SeedConversionError::InvalidDimension {
        cause: Size2DConversionError::InvalidFormat
      })
    )
  }

  #[test]
  fn test_probs_all_zero() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#000000000000000000"),
      Err(SeedConversionError::InvalidProbabilities {
        cause: TileProbsConversionError::AllZeros
      })
    )
  }

  #[test]
  fn test_parsing_a_seed() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#112233445566778899"),
      Ok(Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        Some(TileProbs::try_from([17, 34, 51, 68, 85, 102, 119, 136, 153]).unwrap())
      ))
    )
  }
  #[test]
  fn test_parsing_a_seed_no_tile_probs() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9"),
      Ok(Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        None
      ))
    )
  }
  #[test]
  fn test_serializing_a_seed() {
    assert_eq!(
      &Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        Some(TileProbs::try_from([17, 34, 51, 68, 85, 102, 119, 136, 153]).unwrap())
      )
      .to_string(),
      "ABCD_abcd_1234#6x9#112233445566778899",
    )
  }
  #[test]
  fn test_serializing_a_seed_no_tile_probs() {
    assert_eq!(
      &Seed::new(
        UserString::try_from("ABCD_abcd_1234".to_string()).unwrap(),
        Size2D::new_unchecked(6, 9),
        None
      )
      .to_string(),
      "ABCD_abcd_1234#6x9",
    )
  }
}

mod game_field_test {
  use super::*;

  #[test]
  fn test_ff_probs() {
    let seed = Seed::try_from("none#1x1#ffffffffffffffffff").unwrap();
    let _ = GameField::from_seed(&seed);
  }
}
