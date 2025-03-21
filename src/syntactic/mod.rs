use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(yam);
use yam::{CompUnitParser, Token};
use crate::ast::comp_unit::CompUnit;
use crate::error::Error;

pub struct Analyzer {
  parser: CompUnitParser,
}

impl Analyzer {
  pub fn new() -> Self {
    Self {
      parser: CompUnitParser::new(),
    }
  }

  pub fn parse<'input>(&self, input: &'input str) -> Result<CompUnit, Error> {
    let res: Result<CompUnit, ParseError<usize, Token<'input>, &'static str>> = self.parser.parse(input);
    if res.is_err() {
      let err: String;
      match res.unwrap_err() {
        ParseError::InvalidToken { location } => {
          err = format!("Invalid token found at {}", location)
        }
        ParseError::UnrecognizedEof { location, expected } => {
          err = format!("Unexpected EOF found at {}, expected tokens are:\n{:#?}", location, expected)
        },
        ParseError::UnrecognizedToken { token, .. } => {
          let (location_start, t, location_end) = token;
          err = format!("Token '{}' is not recognized, location: {} to {}", t, location_start, location_end);
        },
        ParseError::ExtraToken { token, .. } => {
          let (location_start, t, location_end) = token;
          err = format!("Token '{}' is extra, location: {} to {}", t, location_start, location_end);
        },
        ParseError::User { error } => {
          err = error.to_string();
        },
      }
      Err(Error::ParseError(err))
    } else {
      Ok(res.unwrap())
    }
  }
}
