use clap::Args;
use diesel::SqliteConnection;
use termimad::MadSkin;

use crate::models::Recipe;

#[derive(Args)]
pub struct PrintArgs {
  /// Integer ID of the recipe to delete
  id: i32,
}

impl PrintArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    match Recipe::find_optional(&self.id, conn)? {
      None => println!("No such recipe: {}", self.id),
      Some(recipe) => {
        let skin = MadSkin::default();
        skin.print_text(&recipe.markdown_string());
      },
    }

    Ok(())
  }
}
