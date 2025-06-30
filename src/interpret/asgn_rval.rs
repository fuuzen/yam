use crate::{ast::{measure::{Measure, MeasureAttr, MeasureAttrValue, MeasureRVal, MeasureUnit, MeasureUnitValue, MeasureValue}, note::{Note, NoteValue}, phrase::{Phrase, PhraseRVal, PhraseValue}, stmt::AsgnRVal, track::{Track, TrackValue}, val::Value}, error::Error, interpret::ctr::RetVal};

use super:: Interpreter;

impl Interpreter {
  /// 翻译 Note 为 NoteValue
  pub fn interpret_note(&mut self, note: &Note) -> Result<NoteValue, Error> {
    let mut notes = vec![];
    for expr in &note.notes {
      let res = self.calc_expr(expr);
      if res.is_err() {
        return Err(res.err().unwrap());
      }
      let v = match res.unwrap() {
        RetVal::Value(Value::Int( int )) => int,
        val => return Err(Error::RuntimeError(format!(
          "expect i32, but found {val}",
        )))
      };
      notes.push(v);
    }
    
    let len  = match note.len.is_some() {
      true => {
        let res = self.calc_expr(note.len.as_ref().unwrap());
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        match res.unwrap() {
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

  /// 翻译 MeasureAttr 为 MeasureAttrValue
  pub fn interpret_measure_attr(&mut self, attr: &MeasureAttr) -> Result<MeasureAttrValue, Error> {
    let mut res = self.calc_expr(&attr.top_num);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let top_num = match res.unwrap() {
      RetVal::Value(Value::Int( int )) => int,
      val => return Err(Error::RuntimeError(format!(
        "expect i32, but found {val}",
      )))
    };

    res = self.calc_expr(&attr.bottom_num);
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let bottom_num = match res.unwrap() {
      RetVal::Value(Value::Int( int )) => int,
      val => return Err(Error::RuntimeError(format!(
        "expect i32, but found {val}",
      )))
    };

    res = self.calc_expr(attr.tempo.as_ref().unwrap());
    if res.is_err() {
      return Err(res.err().unwrap());
    }
    let tempo = match res.unwrap() {
      RetVal::Value(Value::Int( int )) => Some(int),
      val => return Err(Error::RuntimeError(format!(
        "expect i32, but found {val}",
      )))
    };

    Ok(MeasureAttrValue{top_num, bottom_num, tempo})
  }

  /// 翻译 Measure 为 MeasureValue
  pub fn interpret_measure(&mut self, measure: &Measure) -> Result<MeasureValue, Error> {
    let mut content = vec![];
    for unit in &measure.content {
      let unit_val = match unit {
        MeasureUnit::Note( note ) => {
          let res = self.interpret_note(note);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          MeasureUnitValue::NoteValue(res.unwrap())
        },
        MeasureUnit::Rest => MeasureUnitValue::Rest,
        MeasureUnit::TimeDilation => MeasureUnitValue::TimeDilation,
        MeasureUnit::TimeCompression => MeasureUnitValue::TimeCompression,
      };
      content.push(unit_val);
    }

    let attr  = match &measure.attr {
      Some( attr ) => {
        let res = self.interpret_measure_attr(&attr);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Some(res.unwrap())
      },
      None => None,
    };
    Ok(MeasureValue{content, attr})
  }

  /// 翻译 Phrase 为 PhraseValue
  pub fn interpret_phrase(&mut self, phrase: &Phrase) -> Result<PhraseValue, Error> {
    let mut content = vec![];
    for measure_rval in &phrase.content {
      let measure_val = match measure_rval {
        MeasureRVal::Measure( measure ) => {
          let res = self.interpret_measure(measure);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          res.unwrap()
        },
        MeasureRVal::LVal( lval ) => {
          match lval.rval.borrow().as_ref().clone().unwrap().get_value() {
            Value::Measure( v ) => v,
            val => return Err(Error::RuntimeError(format!(
              "expect measure, but found {val}",
            )))
          }
        },
        MeasureRVal::FuncCall( func_call ) => {
          let res = self.call_func(func_call);
          if res.is_err() {
            return Err(res.err().unwrap());
          };
          match res.unwrap() {
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

    let attr  = match &phrase.attr {
      Some( attr ) => {
        let res = self.interpret_measure_attr(&attr);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Some(res.unwrap())
      },
      None => None,
    };
    Ok(PhraseValue{content, attr})
  }

  /// 翻译 Phrase 为 PhraseValue
  pub fn interpret_track(&mut self, track: &Track) -> Result<TrackValue, Error> {
    let mut content = vec![];
    for phrase_rval in &track.content {
      let phrase_val = match phrase_rval {
        PhraseRVal::Phrase( phrase ) => {
          let res = self.interpret_phrase(phrase);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          res.unwrap()
        },
        PhraseRVal::LVal( lval ) => {
          match lval.rval.borrow().as_ref().clone().unwrap().get_value() {
            Value::Phrase( v ) => v,
            val => return Err(Error::RuntimeError(format!(
              "expect phrase, but found {val}",
            )))
          }
        },
        PhraseRVal::FuncCall( func_call ) => {
          let res = self.call_func(func_call);
          if res.is_err() {
            return Err(res.err().unwrap());
          };
          match res.unwrap() {
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
    match asgn_rval {
      AsgnRVal::Expr( expr) => self.calc_expr(expr),
      AsgnRVal::Note( note  ) => {
        let res = self.interpret_note(note);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Ok(RetVal::Value(Value::Note(res.unwrap())))
      },
      AsgnRVal::Measure( measure  ) => {
        let res = self.interpret_measure(measure);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Ok(RetVal::Value(Value::Measure(res.unwrap())))
      },
      AsgnRVal::Phrase( phrase  ) => {
        let res = self.interpret_phrase(phrase);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Ok(RetVal::Value(Value::Phrase(res.unwrap())))
      },
      AsgnRVal::Track( track  ) => {
        let res = self.interpret_track(track);
        if res.is_err() {
          return Err(res.err().unwrap());
        }
        Ok(RetVal::Value(Value::Track(res.unwrap())))
      }
    }
  }
}