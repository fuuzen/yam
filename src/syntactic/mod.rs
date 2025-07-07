use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(yam);
use yam::CompUnitParser;
use crate::ast::comp_unit::CompUnit;
use crate::error::Error;

pub struct Analyzer {
  parser: CompUnitParser,
}

/// 语法分析器
impl Analyzer {
  pub fn new() -> Self {
    Self {
      parser: CompUnitParser::new(),
    }
  }

  pub fn parse<'input>(&self, input: &'input str) -> Result<CompUnit, Error> {
    self.parser.parse(input).map_err(
      |err| Error::ParseError(
        match err {
          ParseError::InvalidToken { location } => {
            format!("Invalid token found at {}", location)
          },
          ParseError::UnrecognizedEof { location, expected } => {
            format!("Unexpected EOF found at {}, expected tokens are:\n{:#?}", location, expected)
          },
          ParseError::UnrecognizedToken { token, .. } => {
            let (location_start, t, location_end) = token;
            format!("Token '{}' is not recognized, location: {} to {}", t, location_start, location_end)
          },
          ParseError::ExtraToken { token, .. } => {
            let (location_start, t, location_end) = token;
            format!("Token '{}' is extra, location: {} to {}", t, location_start, location_end)
          },
          ParseError::User { error } => {
            error.to_string()
          },
        }
      )
    )
  }
}
