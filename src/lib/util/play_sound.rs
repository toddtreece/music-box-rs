use anyhow::Result;
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;

use crate::config::{get_config, Config};

const CONFIG: Config = get_config();

pub fn play_sound(path: String) -> Result<Sink> {
  let device = rodio::default_output_device().unwrap();
  let sink = Sink::new(&device);
  sink.append(Decoder::new(BufReader::new(File::open(path)?))?);
  sink.set_volume(CONFIG.volume * 3.0);
  Ok(sink)
}
