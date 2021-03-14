use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
pub type Connection = PooledConnection<SqliteConnectionManager>;
