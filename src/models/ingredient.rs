use diesel::prelude::*;

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
}

impl<'s> IngredientConstructor<'s> {
  model_creates!(Ingredient);
}
