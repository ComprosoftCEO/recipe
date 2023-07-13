use clap::Args;
use diesel::SqliteConnection;

use crate::models::Tag;

#[derive(Args)]
pub struct DeleteArgs {
  /// ID for the tag
  id: String,
}

impl DeleteArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    if !Tag::exists_from_id(&self.id, conn)? {
      println!("No such tag: {}", self.id);
      return Ok(());
    }

    Tag::delete_from_id(&self.id, conn)?;
    Ok(())
  }
}
