mod task;
use log::info;
use music_box::config::get_config;
use std::fs;
use std::sync::mpsc;
use tokio::time::{sleep, Duration};

use task::Task;

#[tokio::main]
async fn main() {
  env_logger::init();
  fs::create_dir_all(get_config().song_dir()).expect("unable to create song directory");
  fs::create_dir_all(get_config().loop_dir()).expect("unable to create loop directory");
  fs::create_dir_all(get_config().speech_dir()).expect("unable to create speech directory");

  let (tx, rx) = mpsc::channel::<Task>();
  let task_tx = tx.clone();

  task::http::start(task_tx).unwrap();

  let mut task = tokio::spawn(async move {
    task::song::start().await.unwrap();
  });

  loop {
    if let Ok(t) = rx.recv() {
      task.abort();
      info!("task selected: {}", t);
      task = tokio::spawn(async move {
        match t {
          Task::Jukebox => task::song::start().await.unwrap(),
          Task::Loop => task::r#loop::start().unwrap(),
          Task::PerfectPitch => task::game::start().await.unwrap(),
        }
      });
    }
    sleep(Duration::from_millis(10)).await;
  }
}
