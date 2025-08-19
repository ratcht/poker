use std::fmt;

// TypeError
#[derive(Debug, Clone)]
pub struct TypeError(pub String);

impl fmt::Display for TypeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TypeError: {}", self.0)
  }
}

impl std::error::Error for TypeError {}


// DeckError
#[derive(Debug, Clone)]
pub struct DeckError(pub String);

impl fmt::Display for DeckError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "DeckError: {}", self.0)
  }
}

impl std::error::Error for DeckError {}


// TableError
#[derive(Debug, Clone)]
pub struct TableError(pub String);

impl fmt::Display for TableError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "TableError: {}", self.0)
  }
}

impl std::error::Error for TableError {}


// StateMachineError
#[derive(Debug, Clone)]
pub struct StateMachineError(pub String);

impl fmt::Display for StateMachineError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "StateMachineError: {}", self.0)
  }
}

impl std::error::Error for StateMachineError {}
