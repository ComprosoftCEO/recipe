use diesel::SqliteConnection;

use crate::models::Recipe;

pub fn list(conn: &mut SqliteConnection) -> super::Result<()> {
  for (recipe, index) in Recipe::all_ordered(conn)?.into_iter().zip(1..) {
    println!("{}: {}", index, recipe.name);
  }

  Ok(())
}
