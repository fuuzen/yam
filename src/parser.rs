use lalrpop_util::lalrpop_mod;
lalrpop_mod!(yam);
use yam::TrackParser;
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

  pub fn parse(&self, input: String) -> Track {
    self.track_parser.parse(&input).unwrap()
  }
}
