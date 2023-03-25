mod button_map;
mod song;
mod task;
mod types;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use music_box::db;
use std::sync::mpsc::Sender;
use types::AppData;

pub use types::Task;

#[actix_web::main]
pub async fn start(task_tx: Sender<Task>) -> Result<()> {
  let db_pool = db::init().unwrap();

  HttpServer::new(move || {
    let button_map = web::scope("/button_map")
      .route("", web::get().to(button_map::list))
      .route("/{button_id}", web::get().to(button_map::get));

    let songs = web::resource("/songs")
      .route(web::get().to(song::list))
      .route(web::post().to(song::download));

    let jukebox = web::scope("/jukebox").service(songs).service(button_map);

    let task = web::scope("/tasks")
      .route("/jukebox", web::get().to(task::jukebox))
      .route("/loop", web::get().to(task::loop_sequencer))
      .route("/perfect_pitch", web::get().to(task::perfect_pitch));

    App::new()
      .data(AppData {
        db_pool: db_pool.clone(),
        task_tx: task_tx.clone(),
      })
      .service(jukebox)
      .service(task)
  })
  .bind("0.0.0.0:8080")?
  .run();

  Ok(())
}
