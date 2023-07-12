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
