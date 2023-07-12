// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        notes_markdown -> Text,
    }
}

diesel::table! {
    recipe_ingredients (recipe_id, ingredient_id) {
        recipe_id -> Integer,
        ingredient_id -> Integer,
        display_order -> Integer,
        quantity -> Text,
        notes_markdown -> Text,
    }
}

diesel::table! {
    recipe_tags (recipe_id, tag_id) {
        recipe_id -> Integer,
        tag_id -> Text,
    }
}

diesel::table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        instructions_markdown -> Text,
        notes_markdown -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Text,
        name -> Text,
        description_markdown -> Text,
    }
}

diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));
diesel::joinable!(recipe_tags -> recipes (recipe_id));
diesel::joinable!(recipe_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredients,
    recipe_tags,
    recipes,
    tags,
);
