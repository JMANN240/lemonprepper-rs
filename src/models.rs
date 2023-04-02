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