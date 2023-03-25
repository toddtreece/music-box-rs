use super::types::{AppData, HttpError, Task};
use actix_web::{web, HttpResponse};
use anyhow::Result;

pub async fn jukebox(data: web::Data<AppData>) -> Result<HttpResponse, HttpError> {
  data.task_tx.clone().send(Task::Jukebox).unwrap();
  Ok(HttpResponse::Ok().finish())
}

pub async fn loop_sequencer(data: web::Data<AppData>) -> Result<HttpResponse, HttpError> {
  data.task_tx.clone().send(Task::Loop).unwrap();
  Ok(HttpResponse::Ok().finish())
}

pub async fn perfect_pitch(data: web::Data<AppData>) -> Result<HttpResponse, HttpError> {
  data.task_tx.clone().send(Task::PerfectPitch).unwrap();
  Ok(HttpResponse::Ok().finish())
}
