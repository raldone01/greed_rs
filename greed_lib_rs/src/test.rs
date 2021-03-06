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
  fn test_seed_no_user_str() {
    assert_eq!(
      Seed::try_from("#12x12"),
      Err(SeedConversionError::UserStringError {
        cause: UserStringError::Empty
      })
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
  fn test_gen_ff_probs() {
    let seed = Seed::try_from("none#1x1#ffffffffffffffffff").unwrap();
    let _ = GameField::from_seed(&seed);
  }
  #[test]
  fn test_parse_len_to_short() {
    let x: Result<GameField, _> =
      serde_json::from_str("{\"vec\":[1,2,0], \"size\":[2, 2], \"player_pos\":[1,1]}");
    assert!(x.is_err());
  }
  #[test]
  fn test_parse_len_to_long() {
    let x: Result<GameField, _> =
      serde_json::from_str("{\"vec\":[1,2,0,4,5], \"size\":[2, 2], \"player_pos\":[1,1]}");
    assert!(x.is_err());
  }
  #[test]
  fn test_parse_player_outside() {
    let x: Result<GameField, _> =
      serde_json::from_str("{\"vec\":[1,2,3,4], \"size\":[2, 2], \"player_pos\":[2,1]}");
    assert!(x.is_err());
  }
  #[test]
  fn test_parse_player_non_zero() {
    let x: Result<GameField, _> =
      serde_json::from_str("{\"vec\":[1,2,3,4], \"size\":[2, 2], \"player_pos\":[1,1]}");
    assert!(x.is_err());
  }
}
