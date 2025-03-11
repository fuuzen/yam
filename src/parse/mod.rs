use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(yam);
use yam::{TrackParser, Token};
use crate::ast::track::Track;

pub struct Parser {
  track_parser: TrackParser,
}

impl Parser {
  pub fn new() -> Self {
    Self {
      track_parser: TrackParser::new(),
    }
  }

  pub fn parse<'input>(&self, input: &'input str) -> Track {
    let res: Result<Track, ParseError<usize, Token<'input>, &'static str>> = self.track_parser.parse(input);
    if res.is_err() {
      /* TODO */
    }
    res.unwrap()
  }
}
