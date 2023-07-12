use diesel::prelude::*;
use diesel::SqliteConnection;
use inquire::validator::ValueRequiredValidator;
use inquire::{Confirm, Editor, Text};
use termimad::MadSkin;

use crate::models::Recipe;
use crate::models::RecipeConstructor;
use crate::subcommand;

const PLACEHOLDER_INSTRUCTIONS: &str = r#"1. Mix flour, milk, and **eggs** together.
2. Slowly mix in the chocolate chips. _Don't stir too much_
3. Add additional steps as needed ...

Bake at 450°F for 20 minutes. Serve hot."#;

#[derive(Debug, Clone)]
pub struct RecipeEditor {
  existing_recipe_id: Option<i32>,
  name: String,
  instructions_markdown: String,
  notes_markdown: String,
}

struct IngredientEntry {}

impl RecipeEditor {
  pub fn new() -> Self {
    Self {
      existing_recipe_id: None,
      name: String::new(),
      instructions_markdown: PLACEHOLDER_INSTRUCTIONS.to_string(),
      notes_markdown: String::new(),
    }
  }

  pub fn from_recipe(recipe: Recipe, conn: &mut SqliteConnection) -> QueryResult<Self> {
    Ok(Self {
      existing_recipe_id: Some(recipe.id),
      name: recipe.name,
      instructions_markdown: recipe.instructions_markdown,
      notes_markdown: recipe.notes_markdown,
    })
  }

  pub fn edit(mut self, conn: &mut SqliteConnection) -> subcommand::Result<()> {
    loop {
      self.name = Text::new("Recipe Name:")
        .with_initial_value(&self.name)
        .with_validator(ValueRequiredValidator::new("Recipe name cannot be empty"))
        .prompt()?;

      self.instructions_markdown = Editor::new("Instructions")
        .with_file_extension(".md")
        .with_predefined_text(&self.instructions_markdown)
        .prompt()?;

      self.notes_markdown = Editor::new("Notes")
        .with_file_extension(".md")
        .with_predefined_text(&self.notes_markdown)
        .prompt()?;

      if Confirm::new("Does everything look good? (Y/N)").prompt()? {
        break;
      }
    }

    if self.existing_recipe_id.is_some() {
      self.update_recipe(conn)?;
      println!("Changes saved!");
    } else {
      let recipe = self.create_recipe(conn)?;
      println!("Created recipe: {} (ID: {})", recipe.name, recipe.id);
    }

    Ok(())
  }

  fn create_recipe(self, conn: &mut SqliteConnection) -> QueryResult<Recipe> {
    let recipe = RecipeConstructor {
      name: &self.name,
      instructions_markdown: &self.instructions_markdown,
      notes_markdown: &self.notes_markdown,
    }
    .insert_recipe(conn)?;

    Ok(recipe)
  }

  fn update_recipe(self, conn: &mut SqliteConnection) -> QueryResult<()> {
    Recipe {
      id: self.existing_recipe_id.unwrap(),
      name: self.name,
      instructions_markdown: self.instructions_markdown,
      notes_markdown: self.notes_markdown,
    }
    .update(conn)?;

    Ok(())
  }
}
