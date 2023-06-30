pub use super::*;

mod seed_test {
  use alloc::format;

  pub use super::*;

  #[test]
  fn test_seed_to_many() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#112233445566778899#1212312"),
      Err(SeedConversionError::UnexpectedHashTag)
    )
  }
  #[test]
  fn test_seed_size_zero_x() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#0x9#112233445566778899#1212312"),
      Err(Size2DConversionError::ZeroSize.into())
    )
  }
  #[test]
  fn test_seed_size_zero_y() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x0#112233445566778899#1212312"),
      Err(Size2DConversionError::ZeroSize.into())
    )
  }
  #[test]
  fn test_seed_size_zero_both() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#0x0#112233445566778899#1212312"),
      Err(Size2DConversionError::ZeroSize.into())
    )
  }
  #[test]
  fn test_seed_size_large() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#ffffffffffffffffx2#112233445566778899#1212312"),
      Err(
        Size2DConversionError::SizeOutOfRange {
          actual_size: 18_446_744_073_709_551_615
        }
        .into()
      )
    )
  }
  #[test]
  fn test_seed_no_user_str() {
    assert_eq!(
      Seed::try_from("#12x12"),
      Err(SeedConversionError::UserStringError {
        source: UserStringError::Empty
      })
    )
  }
  #[test]
  fn test_seed_no_size() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234"),
      Ok(Seed::new(
        UserString::try_from("ABCD_abcd_1234").unwrap(),
        Size2D::DEFAULT_SIZE,
        Some(TileProbs::DEFAULT_TILE_PROBABILITIES) // Could also use None
      ))
    )
  }
  #[test]
  fn test_seed_only_probs() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#112233445566778899"),
      Err(Size2DConversionError::InvalidFormat.into())
    )
  }

  #[test]
  fn test_probs_all_zero() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#000000000000000000"),
      Err(TileProbsConversionError::AllZeros.into())
    )
  }

  #[test]
  fn test_parsing_a_seed() {
    assert_eq!(
      Seed::try_from("ABCD_abcd_1234#6x9#112233445566778899"),
      Ok(Seed::new(
        UserString::try_from("ABCD_abcd_1234").unwrap(),
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
        UserString::try_from("ABCD_abcd_1234").unwrap(),
        Size2D::new_unchecked(6, 9),
        None
      ))
    )
  }
  #[test]
  fn test_serializing_a_seed() {
    assert_eq!(
      format!(
        "{}",
        Seed::new(
          UserString::try_from("ABCD_abcd_1234").unwrap(),
          Size2D::new_unchecked(6, 9),
          Some(TileProbs::try_from([17, 34, 51, 68, 85, 102, 119, 136, 153]).unwrap())
        )
      ),
      "ABCD_abcd_1234#6x9#112233445566778899",
    )
  }
  #[test]
  fn test_serializing_a_seed_no_tile_probs() {
    assert_eq!(
      format!(
        "{}",
        Seed::new(
          UserString::try_from("ABCD_abcd_1234").unwrap(),
          Size2D::new_unchecked(6, 9),
          None
        )
      ),
      "ABCD_abcd_1234#6x9",
    )
  }
}

mod game_field_test {
  use super::*;

  #[test]
  fn test_empty() {
    assert_eq!(
      GameField::try_from(""),
      Err(GameFieldParserError::NoTrailingNewLine)
    )
  }
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

mod greed_test {
  use super::*;

  #[test]
  fn test_invalid_end_time() {
    let _ =
      Greed::load_from_string("{\"utc_finished_ms\":200000000000000000, \"seed\":\"e\"}").unwrap();
    // For now only verify that the Greed doesn't panic or error out
  }

  #[test]
  fn test_empty_str() {
    assert_eq!(
      Greed::load_from_string(""),
      Err(GreedParserError::EmptyString)
    )
  }
  #[test]
  fn test_invalid_json() {
    if let Err(GreedParserError::InvalidMetaDataFromat { .. }) = Greed::load_from_string("{?}") {
    } else {
      assert!(false)
    }
  }
  #[test]
  fn test_no_gf_information() {
    assert_eq!(
      Greed::load_from_string("{}"),
      Err(GreedParserError::MissingGameFieldInformation)
    )
  }
  #[test]
  fn test_invalid_game_field() {
    assert_eq!(
      Greed::load_from_string("123"),
      Err(GreedParserError::GameFieldParserError {
        source: GameFieldParserError::NoTrailingNewLine
      })
    )
  }
}
