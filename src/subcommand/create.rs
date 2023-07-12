use clap::Args;
use diesel::SqliteConnection;
use inquire::validator::ValueRequiredValidator;
use inquire::{Confirm, Editor, Text};
use termimad::MadSkin;

use crate::models::RecipeConstructor;

#[derive(Args)]
pub struct CreateArgs {
  /// Optional name for the recipe
  #[clap(short, long)]
  name: Option<String>,
}

const PLACEHOLDER_INSTRUCTIONS: &str = r#"1. Mix flour, milk, and **eggs** together.
2. Slowly mix in the chocolate chips. _Don't stir too much_
3. Add additional steps as needed ...

Bake at 450°F for 20 minutes. Serve hot."#;

impl CreateArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let mut name = self.name.unwrap_or_default();
    let mut instructions_markdown = PLACEHOLDER_INSTRUCTIONS.to_string();
    let mut notes_markdown = String::new();

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

    let recipe = RecipeConstructor {
      name: &name,
      instructions_markdown: &instructions_markdown,
      notes_markdown: &notes_markdown,
    }
    .insert_recipe(conn)?;

    println!("Created recipe: {} (ID: {})", recipe.name, recipe.id);

    Ok(())
  }
}
