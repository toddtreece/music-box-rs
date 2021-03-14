mod button_map;
mod song;
mod types;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use music_box::db;

#[actix_web::main]
pub async fn start() -> Result<()> {
  let db_pool = db::init().unwrap();

  HttpServer::new(move || {
    let button_map = web::scope("/button_map")
      .route("", web::get().to(button_map::list))
      .route("/{button_id}", web::get().to(button_map::get));

    let songs = web::resource("/songs")
      .route(web::get().to(song::list))
      .route(web::post().to(song::download));

    let jukebox = web::scope("/jukebox").service(songs).service(button_map);

    App::new().data(db_pool.clone()).service(jukebox)
  })
  .bind("0.0.0.0:8080")?
  .run();

  Ok(())
}
