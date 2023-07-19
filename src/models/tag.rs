use diesel::prelude::*;
use gtmpl_derive::Gtmpl;

use crate::schema::tags;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Gtmpl)]
#[diesel(treat_none_as_null = true)]
pub struct Tag {
  pub id: String,
  pub name: String,
  pub description_markdown: String,
}

impl Tag {
  model_base!(order by tags::name.asc());
  has_many!(RecipeTag);
  has_many!(Recipe through RecipeTag, order by recipes::name.asc());
}
