use diesel::SqliteConnection;

use crate::models::Recipe;

pub fn list(conn: &mut SqliteConnection) -> super::Result<()> {
  for recipe in Recipe::all_ordered(conn)? {
    println!("{}", recipe.name);
  }

  Ok(())
}
