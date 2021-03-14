pub mod button_map;
pub mod model;
pub mod song;
mod types;

use crate::config::{get_config, Config};
use anyhow::Result;
use r2d2_sqlite::SqliteConnectionManager;
pub use types::{Connection, Pool};

const CONFIG: Config = get_config();

pub fn init() -> Result<Pool> {
  let manager = SqliteConnectionManager::file(CONFIG.db_path());
  let pool = Pool::new(manager)?;

  song::create_table(pool.get()?)?;
  button_map::create_table(pool.get()?)?;

  Ok(pool)
}
