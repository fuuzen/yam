use core::fmt;

#[derive(Debug)]
pub enum Error {
  ParseError(String),
  SemanticError(String),
  RuntimeError(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::ParseError(msg) => write!(f, "Parse Error: {}", msg),
      Error::SemanticError(msg) => write!(f, "Semantic Error: {}", msg),
      Error::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
    }
  }
}
