use diesel::prelude::*;
use diesel::SqliteConnection;
use inquire::validator::ValueRequiredValidator;
use inquire::{Confirm, Editor, MultiSelect, Select, Text};
use itertools::Itertools;
use std::iter;
use termimad::MadSkin;

use crate::models::IngredientConstructor;
use crate::models::{Ingredient, Recipe, RecipeConstructor, RecipeIngredient, Tag};
use crate::subcommand;

const PLACEHOLDER_INGREDIENTS: &str = include_str!("placeholder-ingredients.txt");
const PLACEHOLDER_INSTRUCTIONS: &str = include_str!("placeholder-instructions.md");

#[derive(Debug, Clone)]
pub struct RecipeEditor {
  existing_recipe_id: Option<i32>,
  name: String,
  ingredients: Vec<IngredientEntry>,
  instructions_markdown: String,
  notes_markdown: String,
  all_tags: Vec<Tag>,
  selected_tags: Vec<TagIndex>,
}

type TagIndex = usize;

#[derive(Debug, Clone)]
struct IngredientEntry {
  ingredient_id: Option<i32>, // If None, will create a new ingredient
  name: String,
  quantity: String,
  notes_markdown: String,
}

impl RecipeEditor {
  pub fn new(conn: &mut SqliteConnection) -> QueryResult<Self> {
    let all_tags = Tag::all_ordered(conn)?;

    Ok(Self {
      existing_recipe_id: None,
      name: String::new(),
      ingredients: Vec::new(),
      instructions_markdown: PLACEHOLDER_INSTRUCTIONS.to_string(),
      notes_markdown: String::new(),
      all_tags,
      selected_tags: Vec::new(),
    })
  }

  pub fn from_recipe(recipe: Recipe, conn: &mut SqliteConnection) -> QueryResult<Self> {
    let loaded_ingredients = recipe
      .get_ingredients_with_metadata(conn)?
      .into_iter()
      .map(|(i, ri)| IngredientEntry {
        ingredient_id: Some(i.id), // If None, will create a new ingredient
        name: i.name,
        quantity: ri.quantity,
        notes_markdown: ri.notes_markdown,
      })
      .collect();

    let all_tags = Tag::all_ordered(conn)?;

    Ok(Self {
      existing_recipe_id: Some(recipe.id),
      name: recipe.name,
      ingredients: loaded_ingredients,
      instructions_markdown: recipe.instructions_markdown,
      notes_markdown: recipe.notes_markdown,
      all_tags,
      selected_tags: Vec::new(),
    })
  }

  pub fn edit(mut self, conn: &mut SqliteConnection) -> subcommand::Result<()> {
    loop {
      self.name = Text::new("Recipe Name:")
        .with_initial_value(&self.name)
        .with_validator(ValueRequiredValidator::new("Recipe name cannot be empty"))
        .prompt()?;

      let ingredients_string = Editor::new("Ingredients")
        .with_file_extension(".md")
        .with_predefined_text(&self.get_ingredients_text())
        .prompt()?;

      self.parse_and_set_ingredients_from_str(&ingredients_string, conn)?;

      self.instructions_markdown = Editor::new("Instructions")
        .with_file_extension(".md")
        .with_predefined_text(&self.instructions_markdown)
        .prompt()?;

      self.notes_markdown = Editor::new("Notes")
        .with_file_extension(".md")
        .with_predefined_text(&self.notes_markdown)
        .prompt()?;

      if !self.all_tags.is_empty() {
        self.selected_tags = MultiSelect::new("Recipe Tags:", self.all_tags.iter().map(|t| &t.name).collect())
          .with_default(&self.selected_tags)
          .raw_prompt()?
          .into_iter()
          .map(|v| v.index)
          .collect();
      }

      self.print_current_state();
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

  fn get_ingredients_text(&self) -> String {
    if self.ingredients.is_empty() {
      return PLACEHOLDER_INGREDIENTS.into();
    }

    self
      .ingredients
      .iter()
      .map(|i| {
        if !i.notes_markdown.is_empty() {
          format!("{}: {}\n- {}", i.quantity, i.name, i.notes_markdown)
        } else {
          format!("{}: {}", i.quantity, i.name)
        }
      })
      .join("\n")
  }

  /// Prompts the user to select the matching ingredients one-by-one
  fn parse_and_set_ingredients_from_str(&mut self, input: &str, conn: &mut SqliteConnection) -> subcommand::Result<()> {
    self.ingredients = Vec::new();

    let mut lines = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).peekable();
    while let Some(line) = lines.next() {
      let (quantity, ingredient) = line.split_once(":").unwrap_or(("", line));
      let (quantity, ingredient) = (quantity.trim(), ingredient.trim());

      // Next line is notes if it starts with "- "
      let notes = match lines.peek() {
        Some(next_line) if next_line.starts_with("- ") => lines.next().unwrap().strip_prefix("- ").unwrap(),
        _ => "",
      };

      let matching_ingredients = Ingredient::find_by_name(ingredient, conn)?;
      let selected_ingredient = if let Some(index) = matching_ingredients.iter().position(|i| i.name == ingredient) {
        // Name matches exactly, use that ingredient
        index + 1
      } else {
        Select::new(
          &format!("Unknown ingredient \"{ingredient}\""),
          iter::once("<New Ingredient>")
            .chain(matching_ingredients.iter().map(|i| i.name.as_str()))
            .collect(),
        )
        .raw_prompt()?
        .index
      };

      self.ingredients.push(IngredientEntry {
        ingredient_id: (selected_ingredient > 0).then(|| matching_ingredients[selected_ingredient - 1].id),
        name: ingredient.to_string(),
        quantity: quantity.to_string(),
        notes_markdown: notes.to_string(),
      });
    }

    Ok(())
  }

  fn print_current_state(&self) {
    let ingredients_str = self.ingredients.iter().map(IngredientEntry::markdown_string).join("\n");

    let notes = if !self.notes_markdown.is_empty() {
      format!("\n**Notes:**\n{}", self.notes_markdown)
    } else {
      "".into()
    };

    let skin = MadSkin::default();
    skin.print_text(&format!(
      "# {}\n---\n## Ingredients\n{}\n\n## Instructions\n{}\n{}",
      self.name, ingredients_str, self.instructions_markdown, notes,
    ));
  }

  /// Create a new recipe in the database
  fn create_recipe(self, conn: &mut SqliteConnection) -> QueryResult<Recipe> {
    conn.transaction(|conn| {
      let recipe = RecipeConstructor {
        name: &self.name,
        instructions_markdown: &self.instructions_markdown,
        notes_markdown: &self.notes_markdown,
      }
      .insert_recipe(conn)?;

      self.handle_ingredients_and_tags(&recipe, conn)?;

      Ok(recipe)
    })
  }

  /// Update an existing recipe in the database
  fn update_recipe(self, conn: &mut SqliteConnection) -> QueryResult<()> {
    conn.transaction(|conn| {
      let recipe = Recipe {
        id: self.existing_recipe_id.unwrap(),
        name: self.name.clone(),
        instructions_markdown: self.instructions_markdown.clone(),
        notes_markdown: self.notes_markdown.clone(),
      }
      .update(conn)?;

      self.handle_ingredients_and_tags(&recipe, conn)?;

      Ok(())
    })
  }

  fn handle_ingredients_and_tags(mut self, recipe: &Recipe, conn: &mut SqliteConnection) -> QueryResult<()> {
    // Create ingredients that don't exist
    for ingredient in self.ingredients.iter_mut().filter(|i| i.ingredient_id.is_none()) {
      let new_ingredient = IngredientConstructor {
        name: &ingredient.name,
        notes_markdown: &ingredient.notes_markdown,
      }
      .insert_ingredient(conn)?;

      // Set the created IDs
      ingredient.ingredient_id = Some(new_ingredient.id);
    }

    // Replace the recipe ingredients
    recipe.delete_all_recipe_ingredients(conn)?;
    RecipeIngredient::insert_list(
      &self
        .ingredients
        .drain(..)
        .zip(1..)
        .map(|(i, display_order)| RecipeIngredient {
          recipe_id: recipe.id,
          ingredient_id: i.ingredient_id.unwrap(), // Will not panic
          display_order,
          quantity: i.quantity,
          notes_markdown: i.notes_markdown,
        })
        .collect(),
      conn,
    )?;

    // Set the tags
    recipe.set_tags_ids(self.selected_tags.iter().map(|index| &self.all_tags[*index].id), conn)?;

    Ok(())
  }
}

impl IngredientEntry {
  pub fn markdown_string(&self) -> String {
    let quantity = self.quantity.trim();
    let ingredient_string = if !quantity.is_empty() {
      format!("{} {}", quantity, self.name)
    } else {
      self.name.to_string()
    };

    let notes_markdown = self.notes_markdown.trim();
    if !notes_markdown.is_empty() {
      format!("- {}\n  - {}", ingredient_string, notes_markdown)
    } else {
      format!("- {}", ingredient_string)
    }
  }
}
