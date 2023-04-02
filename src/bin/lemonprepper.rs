use clap::{Command, Arg};
use lemonprepper_rs::*;

static EDITABLE_TABLES: [&str; 8] = [
    "units",
    "dimensions",
    "recipes",
    "recipe_ingredients",
    "ingredients",
    "units_conversions",
    "dimension_conversions",
    "inventory"
];

fn main() {
    let matches = Command::new("lemonprepper")
        .version("0.1.0")
        .author("JT Raber")
        .about("A way to keep track of food.")
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("read")
                .about("Read entries from the database.")
                .arg(Arg::new("table").required(true).value_parser(EDITABLE_TABLES))
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes entries from the database.")
                .arg(Arg::new("table").required(true).value_parser(EDITABLE_TABLES))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("read", sub_matches)) => {
            let table_name: &str = sub_matches.get_one::<String>("table").expect("required").as_str();
            read_table(table_name);
        },
        Some(("delete", sub_matches)) => {
            let table_name: &str = sub_matches.get_one::<String>("table").expect("required").as_str();
            match table_name {
                "units" => {
                    println!("Which unit?");
                },
                _ => ()
            }
        },
        _ => unreachable!("How did you get here?")
    }
}