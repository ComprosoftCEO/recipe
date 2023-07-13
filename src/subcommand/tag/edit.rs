use clap::Args;
use diesel::SqliteConnection;

use crate::models::Tag;

#[derive(Args)]
pub struct EditArgs {
  /// Unique ID associated with the tag
  id: String,

  /// Pretty-print name for the tag
  #[arg(short, long)]
  name: Option<String>,

  /// Markdown description for the tag
  #[arg(short, long)]
  description: Option<String>,
}

impl EditArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let mut tag = match Tag::find_optional(&self.id, conn)? {
      Some(tag) => tag,
      None => {
        println!("No such tag: {}", self.id);
        return Ok(());
      },
    };

    if let Some(name) = self.name {
      tag.name = name;
    }
    if let Some(description_markdown) = self.description {
      tag.description_markdown = description_markdown;
    }
    tag.update(conn)?;

    Ok(())
  }
}
