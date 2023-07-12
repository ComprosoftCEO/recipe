use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

const DATABASE_NAME: &str = "recipes.db";
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
  let mut db = SqliteConnection::establish(DATABASE_NAME)?;
  db.run_pending_migrations(MIGRATIONS)?;
  Ok(())
}
