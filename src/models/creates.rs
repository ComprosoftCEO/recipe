// =======================
//  Create for Primary Key
// =======================
#[macro_export]
macro_rules! model_creates(
  ($output_model:ident) => {
    paste::item! {
      model_creates!($output_model, [<$output_model:snake>]);
    }
  };

  ($output_model:ident, $func_base:ident) => {
    model_creates!(@ $crate::models::$output_model, $func_base);
  };

  (@ $output_model:path, $func_base:ident) => {
    // Get parent
    paste::item! {
      // Create the new entry in the database
      pub fn [<insert_ $func_base>](
        &self,
        conn: &mut diesel::SqliteConnection,
      ) -> diesel::prelude::QueryResult<$output_model> {
        use diesel::prelude::*;
        use diesel::insert_into;

        insert_into(<$output_model as diesel::associations::HasTable>::table())
          .values(self)
          .execute(conn)?;


        <$output_model as diesel::associations::HasTable>::table()
          .find($crate::models::last_insert_rowid())
          .get_result::<$output_model>(conn)
      }
    }
  }
);
