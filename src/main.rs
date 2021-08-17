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
  fs::create_dir_all(get_config().speech_dir()).expect("unable to create speech directory");

  let (tx, mut rx) = mpsc::channel::<String>(32);
  let task_tx = tx.clone();

  task::http::start(task_tx).unwrap();

  tokio::spawn(async move {
    //task::song::start().unwrap();
    //task::r#loop::start().unwrap();
    task::game::start().await.unwrap();
  });

  while let Some(message) = rx.recv().await {
    info!("task selected: {}", message);
  }
}
