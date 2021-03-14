mod task;
use log::info;
use music_box::config::get_config;
use std::fs;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
  env_logger::init();
  fs::create_dir_all(get_config().song_dir()).expect("unable to create song directory");
  fs::create_dir_all(get_config().loop_dir()).expect("unable to create loop directory");

  let (tx, mut rx) = mpsc::channel(32);
  let song_tx = tx.clone();

  task::http::start().unwrap();

  tokio::spawn(async move {
    song_tx.send("starting jukebox task").await.unwrap();
    task::song::start().unwrap();
  });

  while let Some(message) = rx.recv().await {
    info!("{}", message);
  }
}
