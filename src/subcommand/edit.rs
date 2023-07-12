use clap::Args;
use diesel::SqliteConnection;

use crate::models::Recipe;
use crate::ui::RecipeEditor;

#[derive(Args)]
pub struct EditArgs {
  /// Integer ID of the recipe to delete
  id: i32,
}

impl EditArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let recipe = match Recipe::find_optional(&self.id, conn)? {
      None => {
        println!("No such recipe: {}", self.id);
        return Ok(());
      },

      Some(recipe) => recipe,
    };

    RecipeEditor::from_recipe(recipe, conn)?.edit(conn)
  }
}
