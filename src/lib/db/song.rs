use super::model::Song;
use super::types::Connection;
use anyhow::Result;
use rusqlite::{params, NO_PARAMS};

pub fn create_table(conn: Connection) -> Result<()> {
  conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS songs (
         id              INTEGER PRIMARY KEY AUTOINCREMENT,
         key             TEXT NOT NULL UNIQUE,
         title           TEXT NOT NULL UNIQUE,
         created         TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
         updated         TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      );
      PRAGMA synchronous = OFF;",
  )?;

  Ok(())
}

pub fn list(conn: Connection) -> Result<Vec<Song>> {
  let mut stmt = conn.prepare(
    "SELECT id, key, title, created, updated 
     FROM songs 
     ORDER BY id",
  )?;

  let results = stmt
    .query_map(NO_PARAMS, |row| {
      Ok(Song {
        id: row.get(0)?,
        key: row.get(1)?,
        title: row.get(2)?,
        created: row.get(3)?,
        updated: row.get(4)?,
      })
    })
    .map(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Song>>()))?;

  results
}

pub fn insert(conn: Connection, key: String, title: String) -> Result<()> {
  let mut stmt = conn.prepare("INSERT OR IGNORE INTO songs (key, title) VALUES (?1, ?2)")?;
  stmt.execute(params![key, title])?;
  Ok(())
}
