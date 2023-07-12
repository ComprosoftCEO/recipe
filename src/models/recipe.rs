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

  pub fn markdown_string(&self) -> String {
    RecipeConstructor {
      name: &self.name,
      instructions_markdown: &self.instructions_markdown,
      notes_markdown: &self.notes_markdown,
    }
    .markdown_string()
  }
}

impl<'s> RecipeConstructor<'s> {
  model_creates!(Recipe);

  pub fn markdown_string(&self) -> String {
    let instructions = self.instructions_markdown.trim();
    let notes = self.notes_markdown.trim();

    let notes = if notes.len() > 0 {
      format!("\n**Notes:**\n{notes}")
    } else {
      "".into()
    };

    format!("## {}\n\n{}\n{}", self.name, instructions, notes,)
  }
}
