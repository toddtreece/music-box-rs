use anyhow::Result;
use rodio::{Decoder, Sink};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use tokio::sync::watch::{self, Receiver};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::config::{get_config, Config};
use crate::ui::{MusicBox, UI};

const CONFIG: Config = get_config();

pub struct Piano {
  ui: Arc<RwLock<UI>>,
  state: HashMap<usize, Arc<Sink>>,
}

impl Piano {
  async fn note_on(&mut self, pin: usize) -> Result<()> {
    let mut ui = self.ui.write().await;
    ui.on(pin)?;
    drop(ui);

    let path = CONFIG.pitch_path(pin);
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    sink.append(Decoder::new(BufReader::new(File::open(path)?))?);
    sink.set_volume(CONFIG.volume);
    self.state.insert(pin, Arc::new(sink));

    sleep(Duration::from_millis(500)).await;
    Ok(())
  }

  async fn note_off(&mut self, pin: usize) -> Result<()> {
    let mut ui = self.ui.write().await;
    ui.off(pin)?;
    drop(ui);

    if let Some(sink) = self.state.get(&pin) {
      sink.stop();
      self.state.remove(&pin);
    }
    Ok(())
  }
}

pub async fn start(ui: Arc<RwLock<UI>>) -> Result<()> {
  let mut piano = Piano {
    ui: Arc::clone(&ui),
    state: HashMap::new(),
  };

  loop {
    let mut pressed = vec![];

    let mut ui = piano.ui.write().await;
    ui.poll().unwrap();
    pressed.append(&mut ui.pressed().clone());
    drop(ui);

    for i in 0..=11 {
      if pressed.contains(&i) {
        piano.note_on(i).await.unwrap();
      } else {
        piano.note_off(i).await.unwrap();
      }
    }
  }
}
