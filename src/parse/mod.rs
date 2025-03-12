use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(yam);
use yam::{TrackParser, Token};
use crate::ast::track::Track;
use crate::error::Error;

pub struct Parser {
  track_parser: TrackParser,
}

impl Parser {
  pub fn new() -> Self {
    Self {
      track_parser: TrackParser::new(),
    }
  }

  pub fn parse<'input>(&self, input: &'input str) -> Result<Track, Error> {
    let res: Result<Track, ParseError<usize, Token<'input>, &'static str>> = self.track_parser.parse(input);
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
