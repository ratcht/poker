use std::fmt;

// Define a custom error type
#[derive(Debug, Clone)]
pub struct TypeError(pub String);

impl fmt::Display for TypeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TypeError: {}", self.0)
  }
}

impl std::error::Error for TypeError {}
