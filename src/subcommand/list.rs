use clap::Args;
use diesel::SqliteConnection;
use itertools::Itertools;
use termimad::MadSkin;

use crate::models::Recipe;
use crate::models::Tag;

#[derive(Args)]
pub struct ListArgs;

impl ListArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let recipes_with_tags: Vec<(Recipe, Vec<Tag>)> = Recipe::all_ordered(conn)?
      .into_iter()
      .map(|r| -> super::Result<_> {
        let tags = r.get_tags_ordered(conn)?;
        Ok((r, tags))
      })
      .collect::<Result<_, _>>()?;

    let table_header = "| ID | Recipe Name | Tags |\n| -:|:- |:- |";
    let table_body = recipes_with_tags
      .into_iter()
      .map(|(recipe, tags)| {
        format!(
          "|{}|{}|{}|",
          recipe.id,
          recipe.name,
          tags.into_iter().map(|t| t.name).join(", ")
        )
      })
      .join("\n");

    let markdown = format!("{}\n{}", table_header, table_body);

    let skin = MadSkin::default();
    skin.print_text(&markdown);

    Ok(())
  }
}
