#[macro_use]
mod associations;
#[macro_use]
mod creates;
mod ingredient;
mod many_many_constructor;
mod recipe;
mod recipe_ingredient;
mod recipe_tag;
mod tag;

pub use ingredient::*;
pub use many_many_constructor::*;
pub use recipe::*;
pub use recipe_ingredient::*;
pub use recipe_tag::*;
pub use tag::*;

use diesel::prelude::*;
use diesel::sql_types::Text;

sql_function!(fn last_insert_rowid() -> Integer);
sql_function!(fn lower(input: Text) -> Text);
