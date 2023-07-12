mod models;
mod schema;
mod subcommand;
mod ui;

use clap::Parser;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Opt {
  /// Database file to use
  #[clap(short, long, global = true, default_value_t = String::from("recipes.db"))]
  database_file: String,

  #[clap(subcommand)]
  subcommand: subcommand::OptSubcommand,
}

fn main() -> subcommand::Result<()> {
  let opt: Opt = Opt::parse();

  // Create the database file if it doesn't exist
  let mut conn = SqliteConnection::establish(&opt.database_file)?;
  conn.run_pending_migrations(MIGRATIONS)?;

  opt.subcommand.execute(&mut conn)?;

  Ok(())
}
