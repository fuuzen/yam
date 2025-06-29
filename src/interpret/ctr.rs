use midi_file::file::Track;

use crate::ast::{measure::Measure, note::Note, phrase::Phrase};

#[derive(Debug)]
pub enum RetVal {
  Int(i32),
  Note(Note),
  Measure(Measure),
  Phrase(Phrase),
  Track(Track),
  Void,
}

#[derive(Debug)]
pub enum Ctr {
  Return(RetVal),
  Continue,
  Break,
  None
}