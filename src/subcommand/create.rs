use clap::Args;
use diesel::SqliteConnection;

use crate::ui::RecipeEditor;

#[derive(Args)]
pub struct CreateArgs;

impl CreateArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    RecipeEditor::new(conn)?.edit(conn)
  }
}
