use anyhow::Result;
use log::{debug, info};
use music_box::config::{get_config, Config};
use music_box::db::{self, button_map, Pool};
use music_box::ui::{MusicBox, UI};
use rodio::{Decoder, Sink};
use std::fs::File;
use std::io::BufReader;

const CONFIG: Config = get_config();

fn create_sink(path: String) -> Result<Sink> {
  let device = rodio::default_output_device().unwrap();
  let sink = Sink::new(&device);
  sink.append(Decoder::new(BufReader::new(File::open(path)?))?);
  sink.set_volume(CONFIG.volume);
  Ok(sink)
}

struct SongList {
  pub cursor: Option<usize>,
  pub current: Option<Sink>,
  db: Pool,
}

impl SongList {
  pub fn new() -> Self {
    Self {
      current: None,
      cursor: None,
      db: db::init().unwrap(),
    }
  }

  pub fn play(&mut self, pin: usize) -> Result<bool> {
    let last = self.cursor;
    self.stop();

    match last {
      Some(last_cursor) if last_cursor == pin => Ok(false),
      _ => {
        let path = CONFIG.song_path(button_map::get(self.db.clone().get()?, pin as i64)?.key);
        info!("playing song: {}", path);
        let sink = create_sink(path)?;
        self.cursor = Some(pin);
        self.current = Some(sink);
        Ok(true)
      }
    }
  }

  pub fn stop(&mut self) {
    if let Some(ref current) = self.current {
      current.stop();
    }
    self.current = None;
    self.cursor = None;
  }
}

pub fn start() -> Result<()> {
  let mut ui = UI::new();
  let mut songs = SongList::new();
  let _jukebox = create_sink(CONFIG.speech_path("jukebox".to_owned()))?;

  loop {
    ui.poll()?;
    let pressed = ui.newly_pressed();

    if let Some(pin) = pressed.get(0) {
      debug!("button pressed: {}", *pin);
      ui.clear()?;

      if songs.play(*pin)? {
        ui.on(*pin)?;
      }
    }

    if let Some(ref song) = songs.current {
      if song.empty() {
        songs.stop();
        ui.clear()?;
      }
    }
  }
}
