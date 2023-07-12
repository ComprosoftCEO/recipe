use clap::Args;
use diesel::SqliteConnection;

use crate::ui::RecipeEditor;

#[derive(Args)]
pub struct CreateArgs {
  /// Optional name for the recipe
  #[clap(short, long)]
  name: Option<String>,
}

impl CreateArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    RecipeEditor::new().edit(conn)
  }
}
