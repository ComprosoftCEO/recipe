use convert_case::{Case, Casing};
use diesel::prelude::*;
use gtmpl_derive::Gtmpl;
use itertools::Itertools;

use crate::models::{Ingredient, RecipeIngredient};
use crate::schema::recipes;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Gtmpl)]
#[diesel(treat_none_as_null = true)]
pub struct Recipe {
  pub id: i32,
  pub name: String,
  pub instructions_markdown: String,
  pub notes_markdown: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipes)]
pub struct RecipeConstructor<'s> {
  pub name: &'s str,
  pub instructions_markdown: &'s str,
  pub notes_markdown: &'s str,
}

impl Recipe {
  model_base!(order by recipes::name.asc());

  has_many!(RecipeIngredient);
  has_many!(Ingredient through RecipeIngredient, order by recipe_ingredients::display_order.asc());

  has_many!(RecipeTag);
  has_many!(Tag through RecipeTag, order by tags::name.asc());

  pub fn get_ingredients_with_metadata(
    &self,
    conn: &mut SqliteConnection,
  ) -> QueryResult<Vec<(Ingredient, RecipeIngredient)>> {
    use crate::schema::ingredients::dsl::ingredients;
    use crate::schema::recipe_ingredients::dsl::{display_order, recipe_id, recipe_ingredients};

    ingredients
      .inner_join(recipe_ingredients)
      .filter(recipe_id.eq(self.id))
      .order_by(display_order.asc())
      .get_results::<(Ingredient, RecipeIngredient)>(conn)
  }

  pub fn markdown_string(&self, conn: &mut SqliteConnection) -> QueryResult<String> {
    let ingredients_str = self
      .get_ingredients_with_metadata(conn)?
      .into_iter()
      .map(|(i, ri)| i.markdown_string(&ri))
      .join("\n");

    let notes = if self.notes_markdown.len() > 0 {
      format!("\n**Notes:**\n{}", self.notes_markdown)
    } else {
      "".into()
    };

    Ok(format!(
      "# {}\n---\n## Ingredients\n{}\n\n## Instructions\n{}\n{}",
      self.name, ingredients_str, self.instructions_markdown, notes,
    ))
  }

  pub fn get_filename(&self) -> String {
    format!("{}.md", self.name.to_case(Case::Kebab))
  }
}

impl<'s> RecipeConstructor<'s> {
  model_creates!(Recipe);
}
