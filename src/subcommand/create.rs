use diesel::SqliteConnection;
use inquire::validator::ValueRequiredValidator;
use inquire::{Editor, Text};

use crate::models::RecipeConstructor;

const PLACEHOLDER_INSTRUCTIONS: &str = r#"1. Mix flour, milk, and **eggs** together.
2. Slowly mix in the chocolate chips. _Don't stir too much_
3. Add additional steps as needed ...

Bake at 450°F for 20 minutes. Serve hot."#;

const PLACEHOLDER_NOTES: &str = r#"Add notes as needed..."#;

pub fn create(conn: &mut SqliteConnection, name: Option<&str>) -> super::Result<()> {
  let name = Text::new("Recipe Name:")
    .with_initial_value(name.unwrap_or_default())
    .with_validator(ValueRequiredValidator::new("Recipe name cannot be empty"))
    .prompt()?;

  let instructions = Editor::new("Instructions")
    .with_file_extension(".md")
    .with_predefined_text(PLACEHOLDER_INSTRUCTIONS)
    .prompt()?;

  let notes = Editor::new("Notes")
    .with_file_extension(".md")
    .with_predefined_text(PLACEHOLDER_NOTES)
    .prompt()?;

  let recipe = RecipeConstructor {
    name: &name,
    instructions_markdown: &instructions,
    notes_markdown: &notes,
  }
  .insert_recipe(conn)?;

  println!("Created recipe: {} (ID: {})", recipe.name, recipe.id);

  Ok(())
}
