use std::fmt;
use crate::types;

struct Card {
  value: types::CardValue,
  suit: types::CardSuit,
}

impl Card {
  fn new(value: i8, suit: i8) -> Result<Self, types::TypeError> {
    Ok(Card {
      value: types::CardValue::new(value)?, // propagate error if invalid
      suit: types::CardSuit::new(suit)?,
    })
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.value.value(), self.suit.symbol())
  }
}
