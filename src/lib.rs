use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn read_table(table_name: &str) -> () {
    match table_name {
        "units" => {
            use crate::schema::units::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = units
                .load::<Unit>(connection)
                .expect("Error loading units");

            println!("Displaying {} units", results.len());
            for unit in results {
                println!("'{}' measures {}", unit.name, unit.dimension_name);
            }
        },
        "dimensions" => {
            use crate::schema::dimensions::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = dimensions
                .load::<Dimension>(connection)
                .expect("Error loading dimensions");

            println!("Displaying {} dimensions", results.len());
            for dimension in results {
                match dimension.base_unit_name {
                    Some(inner_base_unit_name) => println!("{} has base unit {}", dimension.name, inner_base_unit_name),
                    None => println!("{} has no base unit", dimension.name)
                }
            }
        },
        "recipes" => {
            use crate::schema::recipes::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = recipes
                .load::<Recipe>(connection)
                .expect("Error loading recipes");

            println!("Displaying {} recipes", results.len());
            for recipe in results {
                println!("{} has {} servings", recipe.name, recipe.servings)
            }
        },
        "recipe_ingredients" => {
            use crate::schema::recipe_ingredients::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = recipe_ingredients
                .load::<RecipeIngredient>(connection)
                .expect("Error loading recipe ingredients");

            println!("Displaying {} recipe ingredients", results.len());
            for recipe_ingredient in results {
                println!("{} has {} {} {}", recipe_ingredient.recipe_name, recipe_ingredient.ingredient_unit_quantity, recipe_ingredient.ingredient_unit_name, recipe_ingredient.ingredient_name)
            }
        },
        "ingredients" => {
            use crate::schema::ingredients::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = ingredients
                .load::<Ingredient>(connection)
                .expect("Error loading ingredients");

            println!("Displaying {} ingredients", results.len());
            for ingredient in results {
                println!("{}", ingredient.name)
            }
        },
        "inventory_items" => {
            use crate::schema::inventory_items::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = inventory_items
                .load::<InventoryItem>(connection)
                .expect("Error loading inventory items");

            println!("Displaying {} inventory items", results.len());
            for inventory_item in results {
                println!("You have {} {} {}", inventory_item.ingredient_unit_quantity, inventory_item.ingredient_unit_name, inventory_item.ingredient_name)
            }
        },
        "store_items" => {
            use crate::schema::store_items::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = store_items
                .load::<StoreItem>(connection)
                .expect("Error loading store items");

            println!("Displaying {} store items", results.len());
            for store_item in results {
                match store_item.price {
                    Some(inner_price) => println!("The store has {} {} {} as {} for ${}", store_item.ingredient_unit_quantity, store_item.ingredient_unit_name, store_item.ingredient_name, store_item.name, inner_price),
                    None => println!("The store has {} {} {} as {} for an unknown price", store_item.ingredient_unit_quantity, store_item.ingredient_unit_name, store_item.ingredient_name, store_item.name),
                }
            }
        },
        "unit_conversions" => {
            use crate::schema::unit_conversions::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = unit_conversions
                .load::<UnitConversion>(connection)
                .expect("Error loading unit conversions");

            println!("Displaying {} unit conversions", results.len());
            for unit_conversion in results {
                println!("There are {} {} in {} {}", unit_conversion.from_unit_quantity, unit_conversion.from_unit_name, unit_conversion.to_unit_quantity, unit_conversion.to_unit_name)
            }
        },
        "dimension_conversions" => {
            use crate::schema::dimension_conversions::dsl::*;
            
            let connection = &mut establish_connection();
            
            let results = dimension_conversions
                .load::<DimensionConversion>(connection)
                .expect("Error loading dimension conversions");

            println!("Displaying {} dimension conversions", results.len());
            for dimension_conversion in results {
                println!("{} {} of {} is equal to {} {} of {}", dimension_conversion.from_unit_quantity, dimension_conversion.from_unit_name, dimension_conversion.ingredient_name, dimension_conversion.to_unit_quantity, dimension_conversion.to_unit_name, dimension_conversion.ingredient_name)
            }
        },
        _ => ()
    }
}