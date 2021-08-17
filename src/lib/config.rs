use dirs::data_dir;
use log::debug;

pub struct Config {
  pub volume: f32,
  pub audio_format: &'static str,
}

impl Config {
  pub const fn new() -> Self {
    Self {
      volume: 0.025,
      audio_format: "mp3",
    }
  }

  fn dir(&self, name: &str) -> String {
    data_dir()
      .unwrap()
      .join(format!("music_box/{}", name))
      .to_str()
      .unwrap()
      .to_owned()
  }

  pub fn loop_dir(&self) -> String {
    self.dir("loops")
  }

  pub fn song_dir(&self) -> String {
    self.dir("songs")
  }

  pub fn pitch_dir(&self) -> String {
    self.dir("pitch")
  }

  pub fn pitch_path(&self, pin: usize) -> String {
    let path = format!(
      "{}/{}.{}",
      self.pitch_dir(),
      pin.to_string(),
      self.audio_format
    );
    debug!("pitch path: {}", path);
    path
  }

  pub fn speech_dir(&self) -> String {
    self.dir("speech")
  }

  pub fn speech_path(&self, name: String) -> String {
    let path = format!("{}/{}.{}", self.speech_dir(), name, self.audio_format);
    debug!("speech_path: {}", path);
    path
  }

  pub fn song_path(&self, name: String) -> String {
    let path = format!("{}/{}.{}", self.song_dir(), name, self.audio_format);
    debug!("song_path: {}", path);
    path
  }

  pub fn loop_path(&self, name: String) -> String {
    let path = format!("{}/{}.{}", self.loop_dir(), name, self.audio_format);
    debug!("loop_path: {}", path);
    path
  }

  pub fn db_path(&self) -> String {
    let path = self.dir("music_box.sqlite3");
    debug!("db_path: {}", path);
    path
  }
}

pub const fn get_config() -> Config {
  Config::new()
}
