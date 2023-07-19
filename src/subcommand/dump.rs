use clap::Args;
use diesel::SqliteConnection;
use gtmpl_derive::Gtmpl;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use crate::models::{Recipe, Tag};

const INDEX_FILENAME: &str = "README.md";
const RECIPES_FOLDER: &str = "recipes";
const INDEX_TEMPLATE: &str = include_str!("../templates/index.tmpl.md");
const DUMP_SQL_FILENAME: &str = "recipes.sql";

#[derive(Args)]
pub struct DumpArgs {
  /// Directory path to output the markdown and SQL files
  #[clap(short, long, env = "RECIPE_DUMP_PATH")]
  dump_path: PathBuf,

  /// Skips trying to dump the SQL
  ///
  /// Be sure to set this flag if the "sqlite3" command is not in the path
  #[clap(short, long)]
  skip_sql: bool,
}

#[derive(Clone, Gtmpl)]
struct IndexTemplateInput {
  recipes_folder: String,
  recipes: Vec<RecipeWithFilename>,
  tags: Vec<TagWithRecipes>,

  dump_sql_filename: String,
  skip_sql: bool,
}

#[derive(Clone, Gtmpl)]
struct RecipeWithFilename {
  recipe: Recipe,
  filename: String,
}

#[derive(Clone, Gtmpl)]
struct TagWithRecipes {
  tag: Tag,
  recipes: Vec<RecipeWithFilename>,
}

impl DumpArgs {
  pub fn execute(self, conn: &mut SqliteConnection, database_file: &str) -> super::Result<()> {
    // Parent directory
    create_dir_all(&self.dump_path)?;

    // "recipes" directory
    self.create_directory(&[RECIPES_FOLDER])?;

    // Index file
    let index_markdown = self.build_index_template(conn)?;

    let mut index_file = self.create_file(&[INDEX_FILENAME])?;
    write!(index_file, "{}", index_markdown)?;

    // All of the recipe markdown files
    for recipe in Recipe::all(conn)? {
      let recipe_markdown = recipe.markdown_string(conn)?;

      let mut recipe_file = self.create_file(&[RECIPES_FOLDER, &recipe.get_filename()])?;
      write!(recipe_file, "{}", recipe_markdown)?;
    }

    if self.skip_sql {
      println!("Dumped recipes to: {}", self.dump_path.to_string_lossy());
      return Ok(());
    }

    // Example: sqlite3 "recipes.db" ".dump"
    let sql_dump = Command::new("sqlite3").arg(database_file).arg(".dump").output()?.stdout;

    let mut dump_file = self.create_file(&["recipes.sql"])?;
    dump_file.write_all(&sql_dump)?;

    println!(
      "Dumped recipes and database SQL to: {}",
      self.dump_path.to_string_lossy()
    );

    Ok(())
  }

  fn create_directory(&self, paths: &[&str]) -> io::Result<()> {
    let mut path = self.dump_path.clone();
    for p in paths {
      path.push(p);
    }
    Ok(create_dir_all(path)?)
  }

  fn create_file(&self, paths: &[&str]) -> io::Result<File> {
    let mut path = self.dump_path.clone();
    for p in paths {
      path.push(p);
    }
    Ok(File::create(path)?)
  }

  fn build_index_template(&self, conn: &mut SqliteConnection) -> super::Result<String> {
    let all_recipes = Recipe::all_ordered(conn)?;
    let tags = Tag::all_ordered(conn)?
      .into_iter()
      .map(|tag| {
        Ok(TagWithRecipes {
          recipes: tag
            .get_recipes_ordered(conn)?
            .into_iter()
            .map(RecipeWithFilename::from)
            .collect::<Vec<_>>(),
          tag,
        })
      })
      .collect::<super::Result<_>>()?;

    let template_data = IndexTemplateInput {
      recipes_folder: RECIPES_FOLDER.into(),
      recipes: all_recipes.into_iter().map(RecipeWithFilename::from).collect(),
      tags,

      dump_sql_filename: DUMP_SQL_FILENAME.into(),
      skip_sql: self.skip_sql,
    };

    Ok(gtmpl::template(INDEX_TEMPLATE, template_data)?)
  }
}

impl From<Recipe> for RecipeWithFilename {
  fn from(recipe: Recipe) -> Self {
    Self {
      filename: recipe.get_filename(),
      recipe,
    }
  }
}
