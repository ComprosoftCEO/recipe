use clap::Args;
use diesel::SqliteConnection;
use inquire::Confirm;

use crate::models::Recipe;

#[derive(Args)]
pub struct DeleteArgs {
  /// Integer ID of the recipe to delete
  id: i32,

  /// Skip the yes/no prompt
  #[clap(short, long)]
  force: bool,
}

impl DeleteArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    match Recipe::find_optional(&self.id, conn)? {
      None => println!("No such recipe: {}", self.id),
      Some(recipe) => {
        if self.force || Confirm::new(&format!("Really delete recipe \"{}\" (Y/N):", recipe.name)).prompt()? {
          recipe.delete(conn)?;
          println!("Deleted recipe: {}", recipe.id);
        }
      },
    }

    Ok(())
  }
}
