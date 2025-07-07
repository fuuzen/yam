use crate::{ast::{measure::{Measure, MeasureRVal, MeasureUnit, MeasureUnitValue, MeasureValue}, note::{Note, NoteValue}, phrase::{Phrase, PhraseRVal, PhraseValue}, stmt::AsgnRVal, track::{Track, TrackValue}, val::Value}, error::Error, interpret::ctr::RetVal};

use super:: Interpreter;

impl Interpreter {
  /// 翻译 Note 为 NoteValue
  pub fn interpret_note(&mut self, note: &Note) -> Result<NoteValue, Error> {
    let mut notes = vec![];
    for expr in &note.notes {
      let mut v = match self.calc_expr(expr)? {
        RetVal::Value(Value::Int( int )) => vec![int],
        RetVal::Value(Value::Note( note )) => note.notes,
        val => return Err(Error::RuntimeError(format!(
          "expect i32, but found {val}",
        )))
      };
      notes.append(&mut v);
    }
    
    let len  = match note.len.is_some() {
      true => {
        match self.calc_expr(note.len.as_ref().unwrap())? {
          RetVal::Value(Value::Int( int )) => Some(int),
          val => return Err(Error::RuntimeError(format!(
            "expect i32, but found {val}",
          )))
        }
      },
      false => None
    };
    Ok(NoteValue{notes, len})
  }

  /// 翻译 Measure 为 MeasureValue
  pub fn interpret_measure(&mut self, measure: &Measure) -> Result<MeasureValue, Error> {
    let mut content = vec![];
    for unit in &measure.content {
      let unit_val = match unit {
        MeasureUnit::Note( note ) => MeasureUnitValue::NoteValue(self.interpret_note(note)?),
        MeasureUnit::Rest => MeasureUnitValue::Rest,
        MeasureUnit::TimeDilation => MeasureUnitValue::TimeDilation,
        MeasureUnit::TimeCompression => MeasureUnitValue::TimeCompression,
      };
      content.push(unit_val);
    }
    Ok(MeasureValue{content})
  }

  /// 翻译 Phrase 为 PhraseValue
  pub fn interpret_phrase(&mut self, phrase: &Phrase) -> Result<PhraseValue, Error> {
    let mut content = vec![];
    for measure_rval in &phrase.content {
      let measure_val = match measure_rval {
        MeasureRVal::Measure( measure ) => self.interpret_measure(measure)?,
        MeasureRVal::LVal( lval ) => {
          match lval.rval.borrow().as_ref().clone().unwrap().get_value() {
            Value::Measure( v ) => v,
            val => return Err(Error::RuntimeError(format!(
              "expect measure, but found {val}",
            )))
          }
        },
        MeasureRVal::FuncCall( func_call ) => {
          match self.call_func(func_call)? {
            RetVal::Void => return Err(Error::RuntimeError(format!(
              "expect measure, but found void",
            ))),
            RetVal::Value( v ) => match v {
              Value::Measure( v ) => v,
              val => return Err(Error::RuntimeError(format!(
                "expect measure, but found {val}",
              )))
            }
          }
        },
      };
      content.push(measure_val);
    }
    Ok(PhraseValue{content})
  }

  /// 翻译 Phrase 为 PhraseValue
  pub fn interpret_track(&mut self, track: &Track) -> Result<TrackValue, Error> {
    let mut content = vec![];
    for phrase_rval in &track.content {
      let phrase_val = match phrase_rval {
        PhraseRVal::Phrase( phrase ) => self.interpret_phrase(phrase)?,
        PhraseRVal::LVal( lval ) => {
          match lval.rval.borrow().as_ref().clone().unwrap().get_value() {
            Value::Phrase( v ) => v,
            val => return Err(Error::RuntimeError(format!(
              "expect phrase, but found {val}",
            )))
          }
        },
        PhraseRVal::FuncCall( func_call ) => {
          match self.call_func(func_call)? {
            RetVal::Void => return Err(Error::RuntimeError(format!(
              "expect phrase, but found void",
            ))),
            RetVal::Value( v ) => match v {
              Value::Phrase( v ) => v,
              val => return Err(Error::RuntimeError(format!(
                "expect phrase, but found {val}",
              )))
            }
          }
        },
      };
      content.push(phrase_val);
    }

    Ok(TrackValue{content})
  }

  /// 翻译右值表达式
  pub fn interpret_asgn_rval(&mut self, asgn_rval: &AsgnRVal) -> Result<RetVal, Error> {
    Ok(RetVal::Value(
      match asgn_rval {
        AsgnRVal::Expr( expr) => return self.calc_expr(expr),
        AsgnRVal::Note( note  ) => Value::Note(self.interpret_note(note)?),
        AsgnRVal::Measure( measure  ) => Value::Measure(self.interpret_measure(measure)?),
        AsgnRVal::Phrase( phrase  ) => Value::Phrase(self.interpret_phrase(phrase)?),
        AsgnRVal::Track( track  ) => Value::Track(self.interpret_track(track)?)
      }
    ))
  }
}