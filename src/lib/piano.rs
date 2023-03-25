use anyhow::Result;
use rodio::{Decoder, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use crate::config::{get_config, Config};

const CONFIG: Config = get_config();

pub struct Piano {
  state: HashMap<usize, Arc<Sink>>,
}

impl Piano {
  pub fn new() -> Self {
    Self {
      state: HashMap::new(),
    }
  }

  pub fn note_on(&mut self, pin: usize) -> Result<()> {
    let path = CONFIG.pitch_path(pin);
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    sink.append(Decoder::new(BufReader::new(File::open(path)?))?);
    sink.set_volume(CONFIG.volume);
    self.state.insert(pin, Arc::new(sink));
    Ok(())
  }

  pub fn note_off(&mut self, pin: usize) -> Result<()> {
    if let Some(sink) = self.state.get(&pin) {
      sink.stop();
      self.state.remove(&pin);
    }
    Ok(())
  }
}
