use super::model::ButtonMap;
use super::types::Connection;
use anyhow::Result;
use rusqlite::{named_params, NO_PARAMS};

pub fn create_table(conn: Connection) -> Result<()> {
  conn.execute_batch(
    "CREATE TABLE IF NOT EXISTS button_map (
         id              INTEGER PRIMARY KEY AUTOINCREMENT,
         key             TEXT NOT NULL,
         button_id       INTEGER NOT NULL, 
         created         TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      );
      CREATE INDEX IF NOT EXISTS idx_button_created_sort ON button_map (created DESC);
      CREATE INDEX IF NOT EXISTS idx_button_id_sort ON button_map (button_id, created DESC);
      PRAGMA synchronous = OFF;",
  )?;

  Ok(())
}

pub fn list(conn: Connection) -> Result<Vec<ButtonMap>> {
  let mut stmt = conn.prepare(
    "SELECT DISTINCT bm.id, bm.key, bm.button_id, bm.created, s.title
     FROM (SELECT FIRST_VALUE(id) OVER(PARTITION BY button_id ORDER BY created DESC) as id FROM button_map) b
     JOIN button_map bm ON b.id = bm.id
     JOIN songs s ON bm.key = s.key
     ORDER BY bm.button_id;"
  )?;

  let results = stmt
    .query_map(NO_PARAMS, |row| {
      Ok(ButtonMap {
        id: row.get(0)?,
        key: row.get(1)?,
        button_id: row.get(2)?,
        created: row.get(3)?,
        title: row.get(4)?,
      })
    })
    .map(|mapped_rows| {
      Ok(
        mapped_rows
          .map(|row| row.unwrap())
          .collect::<Vec<ButtonMap>>(),
      )
    })?;

  results
}

pub fn get(conn: Connection, button_id: i64) -> Result<ButtonMap> {
  let mut stmt = conn.prepare(
    "SELECT bm.id, bm.key, bm.button_id, bm.created, s.title
     FROM button_map bm
     JOIN songs s ON s.key = bm.key
     WHERE bm.button_id = :button_id
     ORDER BY bm.created DESC LIMIT 1",
  )?;

  let button_map = stmt.query_row_named(named_params! {":button_id": button_id}, |row| {
    Ok(ButtonMap {
      id: row.get(0)?,
      key: row.get(1)?,
      button_id: row.get(2)?,
      created: row.get(3)?,
      title: row.get(4)?,
    })
  })?;

  Ok(button_map)
}

pub fn insert(conn: Connection, key: String, button_id: i64) -> Result<()> {
  let mut stmt =
    conn.prepare("INSERT OR IGNORE INTO button_map (key, button_id) VALUES (:key, :button_id)")?;

  stmt.execute_named(named_params! {
      ":key": key,
      ":button_id": button_id,
  })?;

  Ok(())
}
