table! {
    ingredients (id) {
        id -> Integer,
        name -> Text,
        notes -> Text,
    }
}

table! {
    recipe_ingredients (recipe_id, ingredient_id) {
        recipe_id -> Integer,
        ingredient_id -> Integer,
        quantity -> Text,
        notes -> Text,
    }
}

table! {
    recipe_tags (recipe_id, tag_id) {
        recipe_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        instructions -> Text,
        notes -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

joinable!(recipe_ingredients -> ingredients (ingredient_id));
joinable!(recipe_ingredients -> recipes (recipe_id));
joinable!(recipe_tags -> recipes (recipe_id));
joinable!(recipe_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredients,
    recipe_tags,
    recipes,
    tags,
);
