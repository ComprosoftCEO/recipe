use clap::Args;
use diesel::SqliteConnection;
use std::io::Write;
use tempfile::Builder;
use termimad::MadSkin;

use crate::models::Recipe;

#[derive(Args)]
pub struct PrintArgs {
  /// Integer ID of the recipe to delete
  id: i32,

  #[clap(short, long)]
  web: bool,
}

impl PrintArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let recipe = match Recipe::find_optional(&self.id, conn)? {
      None => {
        println!("No such recipe: {}", self.id);
        return Ok(());
      },
      Some(recipe) => recipe,
    };

    if !self.web {
      let skin = MadSkin::default();
      skin.print_text(&recipe.markdown_string());
      return Ok(());
    }

    let html = markdown::to_html(&recipe.markdown_string());
    let mut file = Builder::new().suffix(".html").tempfile()?;
    write!(
      file,
      r#"
<html>
  <head>
    <title>{recipe}</title>
  </head>
  <body>{body}</body>
</html>"#,
      recipe = recipe.name,
      body = html
    )?;

    open::that(file.path())?;

    Ok(())
  }
}
