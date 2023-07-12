mod create;
mod list;

pub use create::*;
pub use list::*;

use clap::Subcommand;
use diesel::SqliteConnection;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

#[derive(Subcommand)]
pub enum OptSubcommand {
  /// List all recipes in the database
  List,

  /// TUI to create a new recipe in the database
  Create {
    #[clap(short, long)]
    name: Option<String>,
  },
}

impl OptSubcommand {
  pub fn execute(self, conn: &mut SqliteConnection) -> Result<()> {
    use OptSubcommand::*;
    match self {
      List => list(conn),
      Create { name } => create(conn, name.as_deref()),
    }
  }
}
