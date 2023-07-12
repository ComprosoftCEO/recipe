use diesel::prelude::*;

use crate::schema::recipes;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset)]
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
}

impl<'s> RecipeConstructor<'s> {
  model_creates!(Recipe);
}
