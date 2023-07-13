mod create;
mod delete;
mod edit;
mod list;

use clap::Subcommand;
use diesel::SqliteConnection;

pub use super::Result;

#[derive(Subcommand)]
pub enum TagSubcommand {
  /// List all tags in the database
  List(list::ListArgs),

  /// Create a new tag the database
  Create(create::CreateArgs),

  /// Edit tag details
  Edit(edit::EditArgs),

  /// Delete a tag
  Delete(delete::DeleteArgs),
}

impl TagSubcommand {
  pub fn execute(self, conn: &mut SqliteConnection) -> Result<()> {
    use TagSubcommand::*;
    match self {
      List(args) => args.execute(conn),
      Create(args) => args.execute(conn),
      Edit(args) => args.execute(conn),
      Delete(args) => args.execute(conn),
    }
  }
}
