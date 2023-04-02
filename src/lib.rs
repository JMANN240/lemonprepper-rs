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
        _ => ()
    }
}