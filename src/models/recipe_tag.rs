use diesel::prelude::*;
use gtmpl_derive::Gtmpl;

use crate::models::{ManyToManyConstructor, Recipe, Tag};
use crate::schema::recipe_tags;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, Associations, Gtmpl)]
#[diesel(primary_key(recipe_id, tag_id))]
#[diesel(treat_none_as_null = true)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Tag))]
pub struct RecipeTag {
  pub recipe_id: i32,
  pub tag_id: String,
}

impl RecipeTag {
  model_base!(no update);
  belongs_to!(Recipe);
  belongs_to!(Tag);
}

impl ManyToManyConstructor<Recipe, Tag> for RecipeTag {
  fn new(recipe_id: &i32, tag_id: &String) -> Self {
    Self {
      recipe_id: *recipe_id,
      tag_id: tag_id.clone(),
    }
  }
}

impl ManyToManyConstructor<Tag, Recipe> for RecipeTag {
  fn new(tag_id: &String, recipe_id: &i32) -> Self {
    Self {
      recipe_id: *recipe_id,
      tag_id: tag_id.clone(),
    }
  }
}
