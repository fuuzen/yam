use std::collections::HashMap;

use crate::{ast::{measure::{MeasureAttrValue, MeasureUnitValue}, note::NoteValue, score::{ChannelStmt, Score, SetChannelInstrument, SetChannelTrack}, track::TrackRVal, val::Value}, error::Error, interpret::ctr::RetVal};

use midi_file::{core::GeneralMidi, MidiFile};
use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
use midi_file::file::QuartersPerMinute;
use midi_file::file::Track as MidiTrack;

use super:: Interpreter;

/// 每一个小节4个四分音符算,每个四分音符所占tick默认为Divison(PPQ)=1024
const MEASURE_TICKS: u32 = 4 * 1024;

/// 默认音符开启/关闭力度
const DEFAULT_VELOCITY: Velocity = Velocity::new(72);

const DEFAULT_TIME_SIGNATURE_CLOCKS: Clocks = Clocks::Quarter;

const DEFAULT_MEASURRE_ATTR_VALUE: MeasureAttrValue = MeasureAttrValue {
  // u8 = 4 as u8;
  top_num: 4,

  // DurationName = DurationName::Quarter;
  bottom_num: 4,

  // 默认tempo,顾名思义代表每分钟的四分音符数量
  // QuartersPerMinute = QuartersPerMinute::new(80 as u8);
  tempo: Some(80)
};

impl Interpreter {
  /// 翻译 Score 块最终生成 midi
  pub fn interpret_score(&mut self, score: &Score) -> Result<MidiFile, Error> {
    let block = score.block.clone();
    let res = self.interpret_block(block);
    if res.is_err() {
      return Err(res.err().unwrap());
    }

    let mut midi_file = MidiFile::new();
    let mut tracks: HashMap<Channel, MidiTrack> = HashMap::new();

    for stmt in &score.channel_stmts {
      match stmt {
        ChannelStmt::SetChannelInstrument(SetChannelInstrument{channel, instrument}) => {
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

          // 获取该 channel 的 track
          let res = tracks.get_mut(&ch);
          if res.is_none() {
            return Err(Error::RuntimeError(format!(
              "channel {ch} hasn't been assigned any track yet",
            )))
          }
          let track = res.unwrap();
          
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
          track.set_general_midi(ch, instr)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        },
        ChannelStmt::SetChannelTrack(SetChannelTrack{channel, track: track_rval}) => {
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

          let mut track = MidiTrack::default();

          // 应用 MeasureAttrValue 的信息
          let apply_time_signature = |attr: &MeasureAttrValue, track: &mut MidiTrack| -> Result<(), Error> {
            let numerator = u8::try_from(attr.top_num)
              .map_err(|_| Error::RuntimeError(format!(
                "numerator of measure attr must between 0 and 255"
              )))?;
            let denominator = match attr.bottom_num {
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
                "denominator of measure attr must be one of 1,2,4,8,16,32,64,128,256,512,1024"
              ))),
            };

            track.push_time_signature(0, numerator, denominator, DEFAULT_TIME_SIGNATURE_CLOCKS)
              .map_err(|e| Error::RuntimeError(e.to_string()))?;

            if attr.tempo.is_some() {
              let tempo_u8 = u8::try_from(attr.tempo.unwrap())
                .map_err(|_| Error::RuntimeError(format!(
                  "tempo of measure attr must between 0 and 255"
                )))?;
              let tempo = QuartersPerMinute::new(tempo_u8);

              track.push_tempo(0, tempo)
                .map_err(|e| Error::RuntimeError(e.to_string()))?;
            }

            Ok(())
          };

          let mut attr = DEFAULT_MEASURRE_ATTR_VALUE;
          apply_time_signature(&attr, &mut track)?;

          for phrase_val in &track_val.content {
            let attr_bak = attr.clone();
            if phrase_val.attr.is_some() {
              attr = phrase_val.attr.as_ref().unwrap().clone();
              apply_time_signature(&attr, &mut track)?;
            }

            for measure_val in &phrase_val.content {
              let attr_bak = attr.clone();
              if measure_val.attr.is_some() {
                attr = measure_val.attr.as_ref().unwrap().clone();
                apply_time_signature(&attr, &mut track)?;
              }

              let mut notes_to_off: Vec<(u32, NoteValue)> = vec![];
              let mut elapsed_ticks = 0;
              let mut tick_step = MEASURE_TICKS / attr.bottom_num as u32;

              // 模拟midi经过了tick_step个tick,在此期间可能有标记了延长的音符需要关闭
              let elapse = |track: &mut MidiTrack, tick_step: u32, notes_to_off: &mut Vec<(u32, NoteValue)>, elapsed_ticks: &mut u32| -> Result<(), Error> {
                notes_to_off.sort();  // 升序
                let term = *elapsed_ticks + tick_step;
                let mut count = 0;  // 记录需要删掉多少个 note_to_off 记录
                for i in 0..notes_to_off.len() {
                  let (target_elapsed_ticks, note) = &notes_to_off[i];
                  if *target_elapsed_ticks > term {
                    break;
                  }
                  let delta = *target_elapsed_ticks - *elapsed_ticks;
                  for note in &note.notes {
                    track.push_note_off(
                      delta,
                      channel,
                      NoteNumber::new(*note as u8),
                      DEFAULT_VELOCITY
                    ).map_err(|e| Error::RuntimeError(e.to_string()))?;
                  }
                  count += 1;
                  *elapsed_ticks += delta;
                }
                notes_to_off.drain(0..count);
                *elapsed_ticks = term;
                Ok(())
              };

              for measure_unit in &measure_val.content {
                match measure_unit {
                  MeasureUnitValue::TimeDilation => tick_step /= 2,
                  MeasureUnitValue::TimeCompression => tick_step *= 2,
                  MeasureUnitValue::Rest => {
                    elapse(&mut track, tick_step, &mut notes_to_off, &mut elapsed_ticks)?;
                  },
                  MeasureUnitValue::NoteValue( note ) => {
                    let mut note_ticks = tick_step;
                    if note.len.is_some() {
                      note_ticks *= note.len.unwrap() as u32;
                    }
                    let target_elapsed_ticks = elapsed_ticks + note_ticks;

                    for note in &note.notes {
                      track.push_note_on(
                        0,
                        channel,
                        NoteNumber::new(*note as u8),
                        DEFAULT_VELOCITY
                      ).map_err(|e| Error::RuntimeError(e.to_string()))?;
                    }

                    notes_to_off.push((target_elapsed_ticks, note.clone()));

                    elapse(&mut track, tick_step, &mut notes_to_off, &mut elapsed_ticks)?;
                  }
                }
              }

              // 恢复被覆盖的属性
              if phrase_val.attr.is_some() {
                attr = attr_bak;
                apply_time_signature(&attr, &mut track)?;
              }
            }

            // 恢复被覆盖的属性
            if phrase_val.attr.is_some() {
              attr = attr_bak;
              apply_time_signature(&attr, &mut track)?;
            }
          }

          tracks.insert(channel, track);
        }
      }
    }

    for (_, track) in tracks {
      midi_file.push_track(track)
        .map_err(|e| Error::RuntimeError(e.to_string()))?;
    }

    Ok(midi_file)
  }
}