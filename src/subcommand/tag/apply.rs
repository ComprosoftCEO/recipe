use clap::Args;
use diesel::{Connection, SqliteConnection};
use inquire::MultiSelect;
use std::collections::HashSet;

use crate::models::{Recipe, Tag};

#[derive(Args)]
pub struct ApplyArgs {
  /// Unique ID associated with the tag
  id: String,
}

impl ApplyArgs {
  pub fn execute(self, conn: &mut SqliteConnection) -> super::Result<()> {
    let tag = match Tag::find_optional(&self.id, conn)? {
      Some(tag) => tag,
      None => {
        println!("No such tag: {}", self.id);
        return Ok(());
      },
    };

    let all_recipes = Recipe::all_ordered(conn)?;
    let tagged_recipes: HashSet<_> = tag.get_recipe_tags(conn)?.into_iter().map(|rt| rt.recipe_id).collect();

    let selected_recipes: Vec<_> = all_recipes
      .iter()
      .enumerate()
      .filter_map(|(index, recipe)| tagged_recipes.contains(&recipe.id).then_some(index))
      .collect();

    let new_selected_recipes = MultiSelect::new(
      &format!("Recipes to tag as \"{}\":", tag.name),
      all_recipes.iter().map(|r| &r.name).collect(),
    )
    .with_default(&selected_recipes)
    .raw_prompt_skippable()?;

    let new_selected_recipes: Vec<_> = match new_selected_recipes {
      None => {
        println!("No changes made");
        return Ok(());
      },
      Some(recipes) => recipes.into_iter().map(|v| v.index).collect(),
    };

    // Transaction to update the selected recipes
    conn.transaction(|conn| {
      //
      tag.set_recipes_ids(new_selected_recipes.iter().map(|index| &all_recipes[*index].id), conn)
    })?;

    if new_selected_recipes.len() != 1 {
      println!("{} recipes tagged as: {}", new_selected_recipes.len(), tag.name);
    } else {
      println!("1 recipe tagged as: {}", tag.name);
    }

    Ok(())
  }
}
