use super::error;


//============ card::Value ============
#[derive(Debug)]
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

impl std::fmt::Display for CardValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self.0 {
      1 => "A",
      2 => "2", 3 => "3", 4 => "4", 5 => "5",
      6 => "6", 7 => "7", 8 => "8", 9 => "9",
      10 => "T",
      11 => "J",
      12 => "Q",
      13 => "K",
      _ => unreachable!(),
    };
    write!(f, "{}", s)
  }
}

//============ card::Suit ============
#[derive(Debug)]
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

impl std::fmt::Display for CardSuit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.symbol())
  }
}
