use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
  pub id: i64,
  pub key: String,
  pub title: String,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ButtonMap {
  pub id: i64,
  pub key: String,
  pub button_id: i64,
  pub created: DateTime<Utc>,
  pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loop {
  pub id: i64,
  pub title: String,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}
