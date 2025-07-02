use std::collections::HashMap;

use crate::{ast::{measure::MeasureUnitValue, score::{Score, ScoreStmt, SetChannelInstrument, SetChannelTrack, SetTimeSignature}, track::TrackRVal, val::Value}, error::Error, interpret::ctr::RetVal};

use midi_file::{core::GeneralMidi, MidiFile};
use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
use midi_file::file::QuartersPerMinute;
use midi_file::file::Track as MidiTrack;

use super:: Interpreter;

/// 每一个小节4个四分音符算,每个四分音符所占tick默认为Divison(PPQ)=1024
const MEASURE_TICKS: u32 = 4 * 1024;

/// 默认音符开启/关闭力度
const DEFAULT_VELOCITY: Velocity = Velocity::new(72);

/// 默认是X/4拍,四分音符为单位
const DEFAULT_TIME_SIGNATURE_DENOMINATOR: i32 = 4;

const DEFAULT_TIME_SIGNATURE_CLOCKS: Clocks = Clocks::Quarter;

impl Interpreter {
  /// 翻译 Score 块最终生成 midi
  pub fn interpret_score(&mut self, score: &Score) -> Result<MidiFile, Error> {
    let block = score.block.clone();
    let res = self.interpret_block(block);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let mut midi_file = MidiFile::new();
    let mut tracks: HashMap<u8, MidiTrack> = HashMap::new();
    let mut deltas: HashMap<u8, u32> = HashMap::new();  // 继承上一次 SetChannelTrack 可能留下来的下一次开启音符的 delta
    let mut meta_track = MidiTrack::default();
    let mut time_sig_denominator = DEFAULT_TIME_SIGNATURE_DENOMINATOR;  // denominator of time signature, default 4

    for stmt in &score.channel_stmts {
      match stmt {
        ScoreStmt::SetChannelInstrument(SetChannelInstrument{channel, instrument}) => {
          // 计算并检查 channel
          let res = self.calc_expr(channel);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          let ch_i32 = match res.unwrap() {
            RetVal::Value(Value::Int( int )) => int,
            val => return Err(Error::RuntimeError(format!(
              "expect i32, but found {val}",
            )))
          };
          let ch_u8 = match u8::try_from(ch_i32).is_ok_and(|v| v<16) {
            true => u8::try_from(ch_i32).unwrap(),
            false => return Err(Error::RuntimeError(format!(
              "channel must between 0 and 15",
            )))
          };
          let ch = Channel::new(ch_u8);

          // // 获取该 channel 的 track
          // let res = tracks.get_mut(&ch_u8);
          // if res.is_none() {
          //   return Err(Error::RuntimeError(format!(
          //     "channel {ch} hasn't been assigned any track yet",
          //   )))
          // }
          // let track = res.unwrap();
          
          // 计算并检查 instrument
          let res = self.calc_expr(instrument);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          let instr_i32 = match res.unwrap() {
            RetVal::Value(Value::Int( int )) => int,
            val => return Err(Error::RuntimeError(format!(
              "expect i32, but found {val}",
            )))
          };
          let instr_u8 = match u8::try_from(instr_i32).is_ok_and(|v| v<128) {
            true => u8::try_from(instr_i32).unwrap(),
            false => return Err(Error::RuntimeError(format!(
              "channel must between 0 and 15",
            )))
          };
          let instr = GeneralMidi::from(instr_u8);

          // 设置 midi 乐器
          meta_track.set_general_midi(ch, instr)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        },

        ScoreStmt::SetChannelTrack(SetChannelTrack{channel, track: track_rval}) => {
          // 计算并检查 channel
          let res = self.calc_expr(channel);
          if res.is_err() {
            return Err(res.err().unwrap());
          }
          let channel_i32 = match res.unwrap() {
            RetVal::Value(Value::Int( int )) => int,
            val => return Err(Error::RuntimeError(format!(
              "expect i32, but found {val}",
            )))
          };
          let channel_u8 = match u8::try_from(channel_i32).is_ok_and(|v| v<16) {
            true => u8::try_from(channel_i32).unwrap(),
            false => return Err(Error::RuntimeError(format!(
              "channel must between 0 and 15",
            )))
          };
          let channel = Channel::new(channel_u8);

          // 翻译 Track 为 TrackValue
          let track_val = match track_rval {
            TrackRVal::Track( track ) => {
              let res = self.interpret_track(track);
              if res.is_err() {
                return Err(res.err().unwrap());
              }
              res.unwrap()
            },
            TrackRVal::LVal( lval ) => {
              match lval.rval.borrow().as_ref().clone().unwrap().get_value() {
                Value::Track( v ) => v,
                val => return Err(Error::RuntimeError(format!(
                  "expect track, but found {val}",
                )))
              }
            },
            TrackRVal::FuncCall( func_call ) => {
              let res = self.call_func(func_call);
              if res.is_err() {
                return Err(res.err().unwrap());
              };
              match res.unwrap() {
                RetVal::Void => return Err(Error::RuntimeError(format!(
                  "expect track, but found void",
                ))),
                RetVal::Value( v ) => match v {
                  Value::Track( v ) => v,
                  val => return Err(Error::RuntimeError(format!(
                    "expect track, but found {val}",
                  )))
                }
              }
            },
          };

          let track_ = tracks.get(&channel_u8).cloned();
          let track_is_new = track_.is_none();
          let mut track = match track_ {
            None => MidiTrack::default(),
            Some(t) => t.clone()
          };
          let mut delta = match track_is_new {
            true => 0,
            false => *deltas.get(&channel_u8).unwrap(),
          };
          let mut notes_to_off: Vec<(u32, u32)> = vec![];
          let mut elapsed_ticks = 0;
          let mut tick_step = MEASURE_TICKS / time_sig_denominator as u32;

          // 模拟midi经过了tick_step个tick,在此期间可能有标记了延长的音符需要关闭
          let elapse = |track: &mut MidiTrack, tick_step: u32, notes_to_off: &mut Vec<(u32, u32)>, elapsed_ticks: &mut u32, delta: &mut u32| -> Result<(), Error> {
            notes_to_off.sort();  // 升序
            let term = *elapsed_ticks + tick_step;
            let mut count = 0;  // 记录需要删掉多少个 note_to_off 记录
            for i in 0..notes_to_off.len() {
              let (target_elapsed_ticks, note) = &notes_to_off[i];
              if *target_elapsed_ticks > term {
                break;
              }
              let delta = *target_elapsed_ticks - *elapsed_ticks;
              track.push_note_off(
                delta,
                channel,
                NoteNumber::new(*note as u8),
                DEFAULT_VELOCITY
              ).map_err(|e| Error::RuntimeError(e.to_string()))?;
              count += 1;
              *elapsed_ticks += delta;
            }
            notes_to_off.drain(0..count);
            *delta += term - *elapsed_ticks;
            *elapsed_ticks = term;
            Ok(())
          };

          for phrase_val in &track_val.content {
            for measure_val in &phrase_val.content {
              for measure_unit in &measure_val.content {
                match measure_unit {
                  MeasureUnitValue::TimeDilation => tick_step /= 2,
                  MeasureUnitValue::TimeCompression => tick_step *= 2,
                  MeasureUnitValue::Rest => {
                    elapse(&mut track, tick_step, &mut notes_to_off, &mut elapsed_ticks, &mut delta)?;
                  },
                  MeasureUnitValue::NoteValue( note ) => {
                    let mut note_ticks = tick_step;
                    if note.len.is_some() {
                      note_ticks *= note.len.unwrap() as u32;
                    }
                    let target_elapsed_ticks = elapsed_ticks + note_ticks;

                    for note in &note.notes {
                      track.push_note_on(
                        delta,
                        channel,
                        NoteNumber::new(*note as u8),
                        DEFAULT_VELOCITY
                      ).map_err(|e| Error::RuntimeError(e.to_string()))?;
                      
                      notes_to_off.push((target_elapsed_ticks, *note as u32));

                      if delta > 0 {
                        delta = 0;
                      }
                    }

                    elapse(&mut track, tick_step, &mut notes_to_off, &mut elapsed_ticks, &mut delta)?;
                  }
                }
              }
            }
          }
          if track_is_new {
            tracks.insert(channel_u8, track);
            deltas.insert(channel_u8, delta);
          } else {
            *tracks.get_mut(&channel_u8).unwrap() = track;
            *deltas.get_mut(&channel_u8).unwrap() = delta;
          }
        },

        ScoreStmt::SetTimeSignature(SetTimeSignature{top_num, bottom_num}) => {
          let numerator = u8::try_from(
            match self.calc_expr(top_num)? {
              RetVal::Value(Value::Int( int )) => int,
              val => return Err(Error::RuntimeError(format!(
                "expect i32, but found {val}",
              )))
            }
          ).map_err(|_| Error::RuntimeError(format!(
            "numerator of time signature must between 0 and 255"
          )))?;

          let denominator = match match self.calc_expr(bottom_num)? {
            RetVal::Value(Value::Int( int )) => {
              time_sig_denominator = int;
              int
            },
            val => return Err(Error::RuntimeError(format!(
              "expect i32, but found {val}",
            )))
          } {
            1 => DurationName::Whole,
            2 => DurationName::Half,
            4 => DurationName::Quarter,
            8 => DurationName::Eighth,
            16 => DurationName::Sixteenth,
            32 => DurationName::D32,
            64 => DurationName::D64,
            128 => DurationName::D128,
            256 => DurationName::D256,
            512 => DurationName::D512,
            1024 => DurationName::D1024,
            _ => return Err(Error::RuntimeError(format!(
              "denominator of time signature must be one of 1,2,4,8,16,32,64,128,256,512,1024"
            ))),
          };

          meta_track.push_time_signature(0, numerator, denominator, DEFAULT_TIME_SIGNATURE_CLOCKS)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        },

        ScoreStmt::SetTempo( expr ) => {
          
          let tempo_u8 = u8::try_from(
            match self.calc_expr(expr)? {
              RetVal::Value(Value::Int( int )) => int,
              val => return Err(Error::RuntimeError(format!(
                "expect i32, but found {val}",
              )))
            }
          ).map_err(|_| Error::RuntimeError(format!(
            "tempo must between 0 and 255"
          )))?;
          let tempo = QuartersPerMinute::new(tempo_u8);

          meta_track.push_tempo(0, tempo)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        }
      }
    }

    midi_file.push_track(meta_track)
      .map_err(|e| Error::RuntimeError(e.to_string()))?;
    for (_, track) in tracks {
      midi_file.push_track(track)
        .map_err(|e| Error::RuntimeError(e.to_string()))?;
    }

    Ok(midi_file)
  }
}