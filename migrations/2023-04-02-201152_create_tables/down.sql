DROP TABLE dimension_conversions;
DROP TABLE unit_conversions;
DROP TABLE store_items;
DROP TABLE inventory;
DROP TABLE ingredients;
DROP TABLE recipe_ingredients;
DROP TABLE recipes;
DROP TABLE units;
DROP TABLE dimensions;

CREATE TABLE dimensions (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE units (
    name TEXT PRIMARY KEY NOT NULL,
    dimension_name TEXT NOT NULL,
    FOREIGN KEY (dimension_name) REFERENCES dimensions (name)
);