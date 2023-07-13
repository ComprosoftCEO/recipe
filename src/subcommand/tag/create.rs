use clap::Args;
use diesel::SqliteConnection;

use crate::models::Tag;

#[derive(Args)]
pub struct CreateArgs {
  /// Unique ID associated with the tag
  id: String,

  /// Pretty-print name for the tag
  #[arg(short, long)]
  name: Option<String>,

  /// Markdown description for the tag
  #[arg(short, long)]
  description: Option<String>,
}

impl CreateArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    if Tag::exists_from_id(&self.id, conn)? {
      println!("Error, existing tag: {}", self.id);
      return Ok(());
    }

    let tag = Tag {
      id: self.id.clone(),
      name: self.name.unwrap_or(self.id),
      description_markdown: self.description.unwrap_or_default(),
    };
    tag.insert(conn)?;

    Ok(())
  }
}
