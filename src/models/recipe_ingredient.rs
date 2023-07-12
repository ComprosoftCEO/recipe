use diesel::prelude::*;

use crate::models::{Ingredient, ManyToManyConstructor, Recipe};
use crate::schema::recipe_ingredients;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Associations)]
#[diesel(primary_key(recipe_id, ingredient_id))]
#[diesel(treat_none_as_null = true)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Ingredient))]
pub struct RecipeIngredient {
  pub recipe_id: i32,
  pub ingredient_id: i32,
  pub display_order: i32,
  pub quantity: String,
  pub notes_markdown: String,
}

impl RecipeIngredient {
  model_base!();
  belongs_to!(Recipe);
  belongs_to!(Ingredient);

  #[inline]
  pub fn new(recipe_id: i32, ingredient_id: i32) -> Self {
    Self {
      recipe_id,
      ingredient_id,
      display_order: 0,
      quantity: String::new(),
      notes_markdown: String::new(),
    }
  }

  /// Note: ingredient ID will need to be filled in later using TUI
  pub fn parse_from_str(input: &str, recipe_id: i32) -> Vec<(&str, RecipeIngredient)> {
    let mut lines = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty()).peekable();

    let mut display_order = 1;
    let mut result = Vec::new();
    while let Some(line) = lines.next() {
      let (quantity, ingredient) = line.split_once(":").unwrap_or(("", line));

      // Next line is notes if it starts with "- "
      let notes = match lines.peek() {
        Some(next_line) if next_line.starts_with("- ") => lines.next().unwrap().strip_prefix("- ").unwrap(),
        _ => "",
      };

      result.push((
        ingredient,
        RecipeIngredient {
          recipe_id,
          ingredient_id: -1, // Populated later
          display_order,
          quantity: quantity.trim().to_string(),
          notes_markdown: notes.trim().to_string(),
        },
      ));
      display_order += 1;
    }

    return result;
  }
}

impl ManyToManyConstructor<Recipe, Ingredient> for RecipeIngredient {
  fn new(recipe_id: &i32, ingredient_id: &i32) -> Self {
    Self::new(*recipe_id, *ingredient_id)
  }
}

impl ManyToManyConstructor<Ingredient, Recipe> for RecipeIngredient {
  fn new(ingredient_id: &i32, recipe_id: &i32) -> Self {
    Self::new(*recipe_id, *ingredient_id)
  }
}
