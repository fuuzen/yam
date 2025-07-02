use crate::ast::func::FuncType;
use crate::ast::measure::{Measure, MeasureRVal, MeasureUnit};
use crate::ast::note::Note;
use crate::ast::phrase::{Phrase, PhraseRVal};
use crate::ast::stmt::AsgnRVal;
use crate::ast::track::Track;
use crate::ast::val::BType;
use crate::error::Error;

use super::Analyzer;


impl Analyzer {
  pub fn note_check(&mut self, note: &Note) -> Result<(), Error> {
    if note.len.is_some() {
      let res = self.expr_check(note.len.as_ref().unwrap(), Some(BType::Int));
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    for expr in &note.notes {
      let res = self.expr_check(expr, Some(BType::Int));
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }
  
  pub fn measure_check(&mut self, measure: &Measure) -> Result<(), Error> {
    for expr in &measure.content {
      match expr {
        MeasureUnit::Note( note ) => self.note_check(note)?,
        _ => {}
      }
    }
    Ok(())
  }
  
  pub fn phrase_check(&mut self, phrase: &Phrase) -> Result<(), Error> {
    for measure_rval in &phrase.content {
      match measure_rval {
        MeasureRVal::Measure( measure ) => self.measure_check(measure)?,
        MeasureRVal::LVal( lval ) => {
          self.lval_check(lval)?;

          let ret_type = lval.rval.borrow().as_ref().unwrap().get_btype();
          if ret_type != BType::Measure {
            return Err(Error::SemanticError(format!("expect measure, but found {ret_type}", )));
          }
        },
        MeasureRVal::FuncCall( func_call ) => {
          let ret_type = self.func_call_check(func_call)?;

          match ret_type {
            FuncType::Void => return Err(Error::SemanticError(format!("expect measure, but found void", ))),
            FuncType::BType( ret_type ) => {
              if ret_type != BType::Measure {
                return Err(Error::SemanticError(format!("expect measure, but found {ret_type}", )));
              }
            }
          }
        }
      }
    }
    Ok(())
  }
  
  pub fn track_check(&mut self, track: &Track) -> Result<(), Error> {
    for phrase_rval in &track.content {
      match phrase_rval {
        PhraseRVal::Phrase( phrase ) => self.phrase_check(phrase)?,
        PhraseRVal::LVal( lval ) => {
          self.lval_check(lval)?;

          let ret_type = lval.rval.borrow().as_ref().unwrap().get_btype();
          if ret_type != BType::Phrase {
            return Err(Error::SemanticError(format!("expect phrase, but found {ret_type}", )));
          }
        },
        PhraseRVal::FuncCall( func_call ) => {
          let ret_type = self.func_call_check(func_call)?;

          match ret_type {
            FuncType::Void => return Err(Error::SemanticError(format!("expect phrase, but found void", ))),
            FuncType::BType( ret_type ) => {
              if ret_type != BType::Phrase {
                return Err(Error::SemanticError(format!("expect phrase, but found {ret_type}", )));
              }
            }
          }
        }
      }
    }
    Ok(())
  }

  pub fn asgn_rval_check(&mut self, asgn_rval: &AsgnRVal, expect_type: BType) -> Result<(), Error> {
    match asgn_rval {
      AsgnRVal::Expr( expr ) => self.expr_check(expr, Some(expect_type))?,
      AsgnRVal::Note( note ) => {
        if expect_type != BType::Note {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found note")));
        }
        self.note_check(note)?;
      }
      AsgnRVal::Measure( measure ) => {
        if expect_type != BType::Measure {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found measure")));
        }
        self.measure_check(measure)?;
      }
      AsgnRVal::Phrase(phrase ) => {
        if expect_type != BType::Phrase {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found phrase")));
        }
        self.phrase_check(phrase)?;
      }
      AsgnRVal::Track( track ) => {
        if expect_type != BType::Track {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found track")));
        }
        self.track_check(track)?;
      }
    }
    Ok(())
  }
}