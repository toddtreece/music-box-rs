mod button_map;
mod song;
mod types;

use actix_web::{web, App, HttpResponse, HttpServer};
use anyhow::Result;
use music_box::db;
use tokio::sync::mpsc::Sender;

#[actix_web::main]
pub async fn start(task_tx: Sender<String>) -> Result<()> {
  let db_pool = db::init().unwrap();

  HttpServer::new(move || {
    let button_map = web::scope("/button_map")
      .route("", web::get().to(button_map::list))
      .route("/{button_id}", web::get().to(button_map::get));

    let songs = web::resource("/songs")
      .route(web::get().to(song::list))
      .route(web::post().to(song::download));

    let jukebox = web::scope("/jukebox").service(songs).service(button_map);

    let task = web::scope("/task")
      .service(web::resource("/jukebox").route(web::get().to(|| {
        //task_tx.clone().send("jukebox".to_owned()).await.unwrap();
        HttpResponse::Ok()
      })))
      .service(web::resource("/loop").route(web::get().to(|| {
        //task_tx.clone().send("loop".to_owned()).await.unwrap();
        HttpResponse::Ok()
      })));

    App::new()
      .data(db_pool.clone())
      .service(jukebox)
      .service(task)
  })
  .bind("0.0.0.0:8080")?
  .run();

  Ok(())
}
