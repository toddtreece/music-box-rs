use super::types::{AppData, HttpError};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use music_box::{
  db::{button_map, model::Song, song},
  util::download_song,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadRequest {
  pub button: i64,
  pub url: String,
}

pub async fn list(data: web::Data<AppData>) -> Result<HttpResponse, HttpError> {
  let pool = data.db_pool.clone();
  let result =
    web::block(move || -> Result<Vec<Song>, anyhow::Error> { song::list(pool.get()?) }).await;

  Ok(HttpResponse::Ok().json(result.map_err(HttpError::from)?))
}

pub async fn download(
  data: web::Data<AppData>,
  payload: web::Json<DownloadRequest>,
) -> Result<HttpResponse, HttpError> {
  let pool = data.db_pool.clone();

  web::block(move || -> Result<(), anyhow::Error> {
    let response = download_song(payload.url.clone())?;
    song::insert(pool.get()?, response.id.clone(), response.title)?;
    button_map::insert(pool.get()?, response.id, payload.button)?;
    Ok(())
  })
  .await?;

  Ok(HttpResponse::Created().finish())
}
