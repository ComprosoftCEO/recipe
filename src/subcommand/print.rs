use clap::Args;
use diesel::SqliteConnection;
use horrorshow::{helper::doctype, html, Raw};
use markdown::Options;
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
      skin.print_text(&recipe.markdown_string(conn)?);
      return Ok(());
    }

    let raw_body = markdown::to_html_with_options(&recipe.markdown_string(conn)?, &Options::gfm())?;
    let (mut file, path) = Builder::new().suffix(".html").tempfile()?.keep()?;
    write!(
      file,
      "{}",
      html! {
        : doctype::HTML;
        html {
          head {
              title : &recipe.name;
              style: Raw(include_str!("styles.css"));
              script: "window.print();";
          }
          body(class = "markdown") {
            : Raw(&raw_body)
          }
        }
      },
    )?;

    open::that(path)?;

    Ok(())
  }
}
