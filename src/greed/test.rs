use super::*;

#[test]
fn greed_from_str() {
  assert_eq!(
    *Greed::try_from("1").unwrap().field(),
    GameField {
      vec: Vec::from([Tile::V1]),
      x_size: 1,
      y_size: 0
    }
  )
}
