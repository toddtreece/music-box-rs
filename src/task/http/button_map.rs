use super::types::HttpError;
use actix_web::{web, HttpResponse};
use anyhow::Result;
use music_box::db::{button_map, model::ButtonMap, Pool};

pub async fn list(db: web::Data<Pool>) -> Result<HttpResponse, HttpError> {
  let pool = db.clone();
  let result =
    web::block(move || -> Result<Vec<ButtonMap>, anyhow::Error> { button_map::list(pool.get()?) })
      .await;

  Ok(HttpResponse::Ok().json(result.map_err(HttpError::from)?))
}

pub async fn get(db: web::Data<Pool>, path: web::Path<i64>) -> Result<HttpResponse, HttpError> {
  let pool = db.clone();
  let result = web::block(move || -> Result<ButtonMap, anyhow::Error> {
    button_map::get(pool.get()?, path.into_inner())
  })
  .await;

  Ok(HttpResponse::Ok().json(result.map_err(HttpError::from)?))
}
