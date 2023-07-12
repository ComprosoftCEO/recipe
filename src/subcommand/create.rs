use diesel::SqliteConnection;

use crate::models::RecipeConstructor;

pub fn create(conn: &mut SqliteConnection, name: Option<&str>) -> super::Result<()> {
  let x = RecipeConstructor {
    name: name.unwrap_or("Test"),
    instructions_markdown: "",
    notes_markdown: "",
  }
  .insert_recipe(conn)?;

  Ok(())
}
