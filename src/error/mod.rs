use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub enum Error {
  ParseError(String),
  SemanticError(String),
  RuntimeError(String),
  InternalError(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::ParseError(msg) => write!(f, "Parse Error: {}", msg),
      Error::SemanticError(msg) => write!(f, "Semantic Error: {}", msg),
      Error::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
      Error::InternalError(msg) => write!(f, "Internal Error: {}", msg),
    }
  }
}

impl From<Error> for io::Error {
  fn from(err: Error) -> io::Error {
    let error_message = match err {
      Error::ParseError(s) => format!("Parse error: {}", s),
      Error::SemanticError(s) => format!("Semantic error: {}", s),
      Error::RuntimeError(s) => format!("Runtime error: {}", s),
      Error::InternalError(s) => format!("Internal error: {}", s),
    };
    io::Error::new(io::ErrorKind::Other, error_message)
  }
}