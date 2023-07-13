use clap::Args;
use diesel::SqliteConnection;
use itertools::Itertools;
use termimad::MadSkin;

use crate::models::Tag;

#[derive(Args)]
pub struct ListArgs;

impl ListArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let table_header = "| Tag ID | Name | Description |\n| -:|:- |:- |";
    let table_body = Tag::all_ordered(conn)?
      .into_iter()
      .map(|tag| format!("|{}|{}|{}|", tag.id, tag.name, tag.description_markdown))
      .join("\n");

    let markdown = format!("{}\n{}", table_header, table_body);
    let skin = MadSkin::default();
    skin.print_text(&markdown);

    Ok(())
  }
}
