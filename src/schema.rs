// @generated automatically by Diesel CLI.

diesel::table! {
    dimension_conversions (ingredient_name, from_unit_name, to_unit_name) {
        ingredient_name -> Text,
        from_unit_name -> Text,
        from_unit_quantity -> Float8,
        to_unit_name -> Text,
        to_unit_quantity -> Float8,
    }
}

diesel::table! {
    dimensions (name) {
        name -> Text,
        base_unit_name -> Nullable<Text>,
    }
}

diesel::table! {
    ingredients (name) {
        name -> Text,
    }
}

diesel::table! {
    inventory_items (ingredient_name) {
        ingredient_name -> Text,
        ingredient_unit_name -> Text,
        ingredient_unit_quantity -> Float8,
    }
}

diesel::table! {
    recipe_ingredients (recipe_name, ingredient_name) {
        recipe_name -> Text,
        ingredient_name -> Text,
        ingredient_unit_name -> Text,
        ingredient_unit_quantity -> Float8,
    }
}

diesel::table! {
    recipes (name) {
        name -> Text,
        servings -> Int4,
    }
}

diesel::table! {
    store_items (name) {
        name -> Text,
        ingredient_name -> Text,
        ingredient_unit_name -> Text,
        ingredient_unit_quantity -> Float8,
        price -> Nullable<Float8>,
    }
}

diesel::table! {
    unit_conversions (from_unit_name, to_unit_name) {
        from_unit_name -> Text,
        from_unit_quantity -> Float8,
        to_unit_name -> Text,
        to_unit_quantity -> Float8,
    }
}

diesel::table! {
    units (name) {
        name -> Text,
        dimension_name -> Text,
    }
}

diesel::joinable!(inventory_items -> ingredients (ingredient_name));
diesel::joinable!(inventory_items -> units (ingredient_unit_name));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_name));
diesel::joinable!(recipe_ingredients -> recipes (recipe_name));
diesel::joinable!(recipe_ingredients -> units (ingredient_unit_name));
diesel::joinable!(store_items -> ingredients (ingredient_name));
diesel::joinable!(store_items -> units (ingredient_unit_name));

diesel::allow_tables_to_appear_in_same_query!(
    dimension_conversions,
    dimensions,
    ingredients,
    inventory_items,
    recipe_ingredients,
    recipes,
    store_items,
    unit_conversions,
    units,
);
