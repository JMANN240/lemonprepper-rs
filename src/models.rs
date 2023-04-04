use diesel::prelude::*;
use crate::schema::units;

#[derive(Queryable)]
pub struct Dimension {
    pub name: String,
    pub base_unit_name: Option<String>
}

#[derive(Queryable)]
pub struct Unit {
    pub name: String,
    pub dimension_name: String
}

#[derive(Insertable)]
#[diesel(table_name = units)]
pub struct NewUnit<'a> {
    pub name: &'a str,
    pub dimension_name: &'a str
}

#[derive(Queryable)]
pub struct Recipe {
    pub name: String,
    pub servings: i32
}

#[derive(Queryable)]
pub struct RecipeIngredient {
    pub recipe_name: String,
    pub ingredient_name: String,
    pub ingredient_unit_name: String,
    pub ingredient_unit_quantity: f64
}

#[derive(Queryable)]
pub struct Ingredient {
    pub name: String
}

#[derive(Queryable)]
pub struct InventoryItem {
    pub ingredient_name: String,
    pub ingredient_unit_name: String,
    pub ingredient_unit_quantity: f64
}

#[derive(Queryable)]
pub struct StoreItem {
    pub name: String,
    pub ingredient_name: String,
    pub ingredient_unit_name: String,
    pub ingredient_unit_quantity: f64,
    pub price: Option<f64>
}

#[derive(Queryable)]
pub struct UnitConversion {
    pub from_unit_name: String,
    pub from_unit_quantity: f64,
    pub to_unit_name: String,
    pub to_unit_quantity: f64
}

#[derive(Queryable)]
pub struct DimensionConversion {
    pub ingredient_name: String,
    pub from_unit_name: String,
    pub from_unit_quantity: f64,
    pub to_unit_name: String,
    pub to_unit_quantity: f64
}