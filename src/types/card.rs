use super::error;


//============ card::Value ============
pub struct CardValue(i8);

impl CardValue {
  pub fn new(value: i8) -> Result<Self, error::TypeError> {
    if value < 1 || value > 13 {
      return Err(error::TypeError("card value out of bounds".to_string()));
    }
    Ok(CardValue(value))
  }

  pub fn value(&self) -> i8 {
    self.0
  }
}


//============ card::Suit ============
pub struct CardSuit(i8);

impl CardSuit {
  pub fn new(value: i8) -> Result<Self, error::TypeError> {
    if value < 0 || value > 3 {
      return Err(error::TypeError("card suit out of bounds".to_string()));
    }
    Ok(CardSuit(value))
  }

  pub fn value(&self) -> i8 {
    self.0
  }

  pub fn symbol(&self) -> &str {
    match self.0 {
      0 => "♣",
      1 => "♠",
      2 => "♥",
      3 => "♦",
      _ => "SuitError"
    }
  }

  pub fn letter(&self) -> &str {
    match self.0 {
      0 => "C",
      1 => "S",
      2 => "H",
      3 => "D",
      _ => "SuitError"
    }
  }
}
