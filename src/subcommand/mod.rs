mod create;
mod delete;
mod dump;
mod edit;
mod list;
mod print;
mod tag;

use clap::Subcommand;
use diesel::SqliteConnection;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Subcommand)]
pub enum OptSubcommand {
  /// List all recipes in the database
  List(list::ListArgs),

  /// TUI to create a new recipe in the database
  Create(create::CreateArgs),

  /// TUI to edit a recipe
  Edit(edit::EditArgs),

  /// Print out a recipe
  Print(print::PrintArgs),

  /// Delete a recipe
  Delete(delete::DeleteArgs),

  /// Dump the entire database to SQL and markdown files
  Dump(dump::DumpArgs),

  /// Manage tags
  Tag {
    #[clap(subcommand)]
    subcommand: tag::TagSubcommand,
  },
}

impl OptSubcommand {
  pub fn execute(self, conn: &mut SqliteConnection, database_file: &str) -> Result<()> {
    use OptSubcommand::*;
    match self {
      List(args) => args.execute(conn),
      Create(args) => args.execute(conn),
      Edit(args) => args.execute(conn),
      Print(args) => args.execute(conn),
      Delete(args) => args.execute(conn),
      Dump(args) => args.execute(conn, database_file),
      Tag { subcommand } => subcommand.execute(conn),
    }
  }
}
