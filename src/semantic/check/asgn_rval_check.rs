use crate::ast::func::FuncType;
use crate::ast::measure::{Measure, MeasureAttr, MeasureRVal, MeasureUnit};
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
      let res = self.expr_check(note.len.as_ref().unwrap(), BType::Int);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    for expr in &note.notes {
      let res = self.expr_check(expr, BType::Int);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }
  
  pub fn measure_attr_check(&mut self, attr: &MeasureAttr) -> Result<(), Error> {
    let mut res = self.expr_check(&attr.top_num, BType::Int);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    res = self.expr_check(&attr.bottom_num, BType::Int);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    if attr.tempo.is_some() {
      res = self.expr_check(attr.tempo.as_ref().unwrap(), BType::Int);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    Ok(())
  }
  
  pub fn measure_check(&mut self, measure: &Measure) -> Result<(), Error> {
    if measure.attr.is_some() {
      let res = self.measure_attr_check(measure.attr.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    for expr in &measure.content {
      match expr {
        MeasureUnit::Note( note ) => {
          let res = self.note_check(note);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        _ => {}
      }
    }
    Ok(())
  }
  
  pub fn phrase_check(&mut self, phrase: &Phrase) -> Result<(), Error> {
    if phrase.attr.is_some() {
      let res = self.measure_attr_check(phrase.attr.as_ref().unwrap());
      if res.is_err() {
        return Err(res.err().unwrap());
      }
    }
    for measure_rval in &phrase.content {
      match measure_rval {
        MeasureRVal::Measure( measure ) => {
          let res = self.measure_check(measure);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        MeasureRVal::LVal( lval ) => {
          let res = self.lval_check(lval);
          if res.is_err() {
            return Err(res.err().unwrap());
          }

          let ret_type = lval.rval.borrow().as_ref().unwrap().get_btype();
          if ret_type != BType::Measure {
            return Err(Error::SemanticError(format!("expect measure, but found {ret_type}", )));
          }
        },
        MeasureRVal::FuncCall( func_call ) => {
          let res = self.func_call_check(func_call);
          if res.is_err() {
            return Err(res.err().unwrap());
          }

          let ret_type = res.as_ref().unwrap();
          match ret_type {
            FuncType::Void => return Err(Error::SemanticError(format!("expect measure, but found void", ))),
            FuncType::BType( ret_type ) => {
              if *ret_type != BType::Measure {
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
        PhraseRVal::Phrase( phrase ) => {
          let res = self.phrase_check(phrase);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
        },
        PhraseRVal::LVal( lval ) => {
          let res = self.lval_check(lval);
          if res.is_err() {
            return Err(res.err().unwrap());
          }

          let ret_type = lval.rval.borrow().as_ref().unwrap().get_btype();
          if ret_type != BType::Phrase {
            return Err(Error::SemanticError(format!("expect phrase, but found {ret_type}", )));
          }
        },
        PhraseRVal::FuncCall( func_call ) => {
          let res = self.func_call_check(func_call);
          if res.is_err() {
            return Err(res.err().unwrap());
          }

          let ret_type = res.as_ref().unwrap();
          match ret_type {
            FuncType::Void => return Err(Error::SemanticError(format!("expect phrase, but found void", ))),
            FuncType::BType( ret_type ) => {
              if *ret_type != BType::Phrase {
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
      AsgnRVal::Expr( expr ) => {
        let res = self.expr_check(expr, expect_type);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      },
      AsgnRVal::Note( note ) => {
        if expect_type != BType::Note {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found note")));
        }
        let res = self.note_check(note);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      }
      AsgnRVal::Measure( measure ) => {
        if expect_type != BType::Measure {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found measure")));
        }
        let res = self.measure_check(measure);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      }
      AsgnRVal::Phrase(phrase ) => {
        if expect_type != BType::Phrase {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found phrase")));
        }
        let res = self.phrase_check(phrase);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      }
      AsgnRVal::Track( track ) => {
        if expect_type != BType::Track {
          return Err(Error::SemanticError(format!("expect {expect_type}, but found track")));
        }
        let res = self.track_check(track);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
      }
    }
    Ok(())
  }
}