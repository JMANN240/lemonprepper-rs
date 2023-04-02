// @generated automatically by Diesel CLI.

diesel::table! {
    dimension_conversions (ingredient_name, from_dimension_name, to_dimension_name) {
        ingredient_name -> Text,
        from_dimension_name -> Text,
        from_dimension_quantity -> Numeric,
        to_dimension_name -> Text,
        to_dimension_quantity -> Numeric,
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
    inventory (ingredient_name) {
        ingredient_name -> Text,
        ingredient_unit_name -> Text,
        ingredient_unit_quantity -> Numeric,
    }
}

diesel::table! {
    recipe_ingredients (recipe_name, ingredient_name) {
        recipe_name -> Text,
        ingredient_name -> Text,
        ingredient_unit_name -> Text,
        ingredient_unit_quantity -> Numeric,
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
        ingredient_unit_quantity -> Numeric,
        price -> Nullable<Numeric>,
    }
}

diesel::table! {
    unit_conversions (from_unit_name, to_unit_name) {
        from_unit_name -> Text,
        from_unit_quantity -> Numeric,
        to_unit_name -> Text,
        to_unit_quantity -> Numeric,
    }
}

diesel::table! {
    units (name) {
        name -> Text,
        dimension_name -> Text,
    }
}

diesel::joinable!(inventory -> ingredients (ingredient_name));
diesel::joinable!(inventory -> units (ingredient_unit_name));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_name));
diesel::joinable!(recipe_ingredients -> recipes (recipe_name));
diesel::joinable!(recipe_ingredients -> units (ingredient_unit_name));
diesel::joinable!(store_items -> ingredients (ingredient_name));
diesel::joinable!(store_items -> units (ingredient_unit_name));

diesel::allow_tables_to_appear_in_same_query!(
    dimension_conversions,
    dimensions,
    ingredients,
    inventory,
    recipe_ingredients,
    recipes,
    store_items,
    unit_conversions,
    units,
);
