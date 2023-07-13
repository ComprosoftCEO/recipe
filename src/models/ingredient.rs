use diesel::prelude::*;

use super::{lower, RecipeIngredient};
use crate::schema::ingredients;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset)]
#[diesel(treat_none_as_null = true)]
pub struct Ingredient {
  pub id: i32,
  pub name: String,
  pub notes_markdown: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = ingredients)]
pub struct IngredientConstructor<'s> {
  pub name: &'s str,
  pub notes_markdown: &'s str,
}

impl Ingredient {
  model_base!(order by ingredients::name.asc());

  // Get all ingredients that match any words of the name
  pub fn find_by_name(input: &str, conn: &mut SqliteConnection) -> QueryResult<Vec<Self>> {
    use crate::schema::ingredients::dsl::{ingredients, name};

    let mut query = ingredients.into_boxed();
    for word in input.split_whitespace() {
      query = query.or_filter(lower(name).like(format!("%{}%", word)));
    }

    query.get_results(conn)
  }

  pub fn markdown_string(&self, recipe_ingredient: &RecipeIngredient) -> String {
    recipe_ingredient.markdown_string(self)
  }
}

impl<'s> IngredientConstructor<'s> {
  model_creates!(Ingredient);
}
