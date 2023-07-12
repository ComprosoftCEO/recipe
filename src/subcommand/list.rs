use clap::Args;
use diesel::SqliteConnection;
use itertools::Itertools;
use termimad::MadSkin;

use crate::models::Recipe;

#[derive(Args)]
pub struct ListArgs {}

impl ListArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let table_header = "| ID | Recipe Name |\n| -:|:-|";
    let table_body = Recipe::all_ordered(conn)?
      .into_iter()
      .map(|recipe| format!("|{}|{}|", recipe.id, recipe.name))
      .join("\n");

    let markdown = format!("{}\n{}", table_header, table_body);

    let skin = MadSkin::default();
    skin.print_text(&markdown);

    Ok(())
  }
}
