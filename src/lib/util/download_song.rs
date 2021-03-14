use crate::config::{get_config, Config};
use anyhow::Result;
use log::{debug, info};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::process::Command;

const CONFIG: Config = get_config();

#[derive(Serialize, Deserialize)]
pub struct YouTubeResponse {
  #[serde(rename = "id")]
  pub id: String,

  #[serde(rename = "title")]
  pub title: String,
}

pub fn download_song(url: String) -> Result<YouTubeResponse> {
  info!("downloading song {} as {}", url, CONFIG.audio_format);
  debug!("PATH: {}", env::var("PATH")?);

  // grab information about the song
  let json_output = Command::new("youtube-dl")
    .env("PATH", env::var("PATH")?)
    .arg("--dump-json")
    .arg(&url)
    .output()
    .expect("youtube-dl command failed. make sure it is in your PATH");

  // actually download the song
  Command::new("youtube-dl")
    .env("PATH", env::var("PATH")?)
    .arg("-x")
    .arg("--audio-format")
    .arg(CONFIG.audio_format)
    .arg("-o")
    .arg(format!("{}/{}", CONFIG.song_dir(), "%(id)s.$(ext)s"))
    .arg(&url)
    .spawn()
    .expect("youtube-dl command failed. make sure it is in your PATH");

  if !json_output.status.success() {
    let err = String::from_utf8_lossy(&json_output.stderr);
    anyhow::bail!("youtube-dl json error: {}", err)
  }

  let out = String::from_utf8_lossy(&json_output.stdout);
  debug!("youtube-dl json output:\n{}", out);
  let response = serde_json::from_str::<YouTubeResponse>(&out)?;

  Ok(response)
}
