use super::types::{AppData, HttpError};
use actix_web::{web, HttpResponse};
use anyhow::Result;
use music_box::db::{button_map, model::ButtonMap};

pub async fn list(data: web::Data<AppData>) -> Result<HttpResponse, HttpError> {
  let pool = data.db_pool.clone();
  let result =
    web::block(move || -> Result<Vec<ButtonMap>, anyhow::Error> { button_map::list(pool.get()?) })
      .await;

  Ok(HttpResponse::Ok().json(result.map_err(HttpError::from)?))
}

pub async fn get(
  data: web::Data<AppData>,
  path: web::Path<i64>,
) -> Result<HttpResponse, HttpError> {
  let pool = data.db_pool.clone();
  let result = web::block(move || -> Result<ButtonMap, anyhow::Error> {
    button_map::get(pool.get()?, path.into_inner())
  })
  .await;

  Ok(HttpResponse::Ok().json(result.map_err(HttpError::from)?))
}
