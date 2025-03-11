use std::env::args;
use std::fs::read_to_string;
use std::io::Result;
use midi_file::MidiFile;
use midi_file::core::{Channel, Clocks, DurationName, NoteNumber, Velocity};
use midi_file::file::{QuartersPerMinute, Track};
use yam::Parser;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  // let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;
  let parser = Parser::new();
  let ast = parser.parse(input);

  let content = format!("{:#?}",ast);
  let mut file = File::create(output)?;
  file.write_all(content.as_bytes())?;

  // let mut midi_file = MidiFile::new();
  // let mut track = Track::default();
  // track.push_tempo(
  //   0,
  //   QuartersPerMinute::new(ast.tempo as u8)
  // ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
  
  // track.push_time_signature(
  //   0,
  //   ast.time_signature.numerator as u8,
  //   match ast.time_signature.denominator {
  //     1 => DurationName::Whole,
  //     2 => DurationName::Half,
  //     4 => DurationName::Quarter,
  //     8 => DurationName::Eighth,
  //     16 => DurationName::Sixteenth,
  //     32 => DurationName::D32,
  //     64 => DurationName::D64,
  //     128 => DurationName::D128,
  //     256 => DurationName::D256,
  //     512 => DurationName::D512,
  //     1024 => DurationName::D1024,
  //     _ => DurationName::Quarter, // TODO handle parse err
  // },
  //   Clocks::Quarter
  // ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

  // for (i, play) in ast.func_def.block.stmts.iter().enumerate() {
  //   let note_number = NoteNumber::new(60 + play.pitch as u8);
  //   let channel = Channel::new(0);
  //   let velocity = Velocity::new(72);
  //   let tick = 4 * 1024 / ast.time_signature.denominator;  // 默认 Divison (PPQ) = 1024
  //   let on_delta_time = if i == 0 {
  //     (tick * play.start) as u32  
  //   } else {
  //     ((play.bar - ast.func_def.block.stmts[i-1].bar) * 4 + play.start - ast.func_def.block.stmts[i-1].end) as u32
  //   };
  //   let off_delta_time = (tick * (play.end - play.start)) as u32;
  //   track.push_note_on(
  //     on_delta_time,  // 紧接着上一个事件
  //     channel,
  //     note_number,
  //     velocity
  //   ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
  //   track.push_note_off(
  //     off_delta_time,
  //     channel,
  //     note_number,
  //     velocity
  //   ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
  // }

  // midi_file.push_track(track)
  //   .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
  // midi_file.save(output).unwrap();
  Ok(())
}
