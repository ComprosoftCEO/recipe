use clap::Args;
use diesel::SqliteConnection;
use inquire::validator::ValueRequiredValidator;
use inquire::{Confirm, Editor, Text};
use termimad::MadSkin;

use crate::models::{Recipe, RecipeConstructor};

#[derive(Args)]
pub struct EditArgs {
  /// Integer ID of the recipe to delete
  id: i32,
}

impl EditArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let recipe = match Recipe::find_optional(&self.id, conn)? {
      None => {
        println!("No such recipe: {}", self.id);
        return Ok(());
      },

      Some(recipe) => recipe,
    };

    let mut name = recipe.name;
    let mut instructions_markdown = recipe.instructions_markdown;
    let mut notes_markdown = recipe.notes_markdown;

    loop {
      name = Text::new("Recipe Name:")
        .with_initial_value(&name)
        .with_validator(ValueRequiredValidator::new("Recipe name cannot be empty"))
        .prompt()?;

      instructions_markdown = Editor::new("Instructions")
        .with_file_extension(".md")
        .with_predefined_text(&instructions_markdown)
        .prompt()?;

      notes_markdown = Editor::new("Notes")
        .with_file_extension(".md")
        .with_predefined_text(&notes_markdown)
        .prompt()?;

      let skin = MadSkin::default();
      skin.print_text(
        &RecipeConstructor {
          name: &name,
          instructions_markdown: &instructions_markdown,
          notes_markdown: &notes_markdown,
        }
        .markdown_string(),
      );

      if Confirm::new("Does everything look good? (Y/N)").prompt()? {
        break;
      }
    }

    Recipe {
      id: recipe.id,
      name,
      instructions_markdown,
      notes_markdown,
    }
    .update(conn)?;

    println!("Changes saved!");

    Ok(())
  }
}
