use actix_web::error::{BlockingError, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "HTTP Error: {}", err.name)]
pub struct HttpError {
  err: anyhow::Error,
}

impl From<BlockingError<anyhow::Error>> for HttpError {
  fn from(err: BlockingError<anyhow::Error>) -> Self {
    match err {
      BlockingError::Canceled => HttpError::from(anyhow::anyhow!("Request cancelled.")),
      BlockingError::Error(e) => HttpError::from(e),
    }
  }
}

impl ResponseError for HttpError {}

impl From<anyhow::Error> for HttpError {
  fn from(err: anyhow::Error) -> HttpError {
    HttpError { err }
  }
}
