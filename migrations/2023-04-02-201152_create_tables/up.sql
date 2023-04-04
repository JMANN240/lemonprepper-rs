DROP TABLE units;
DROP TABLE dimensions;

CREATE TABLE dimensions (
    name TEXT PRIMARY KEY,
    base_unit_name TEXT
);

CREATE TABLE units (
    name TEXT PRIMARY KEY,
    dimension_name TEXT NOT NULL,
    FOREIGN KEY (dimension_name) REFERENCES dimensions (name)
);

ALTER TABLE dimensions ADD CONSTRAINT base_unit_references_units FOREIGN KEY (base_unit_name) REFERENCES units (name);

CREATE TABLE recipes (
    name TEXT PRIMARY KEY,
    servings INTEGER NOT NULL
);

CREATE TABLE ingredients (
    name TEXT PRIMARY KEY
);

CREATE TABLE recipe_ingredients (
    recipe_name TEXT NOT NULL,
    ingredient_name TEXT NOT NULL,
    ingredient_unit_name TEXT NOT NULL,
    ingredient_unit_quantity DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (recipe_name, ingredient_name),
    FOREIGN KEY (recipe_name) REFERENCES recipes (name),
    FOREIGN KEY (ingredient_name) REFERENCES ingredients (name),
    FOREIGN KEY (ingredient_unit_name) REFERENCES units (name)
);

CREATE TABLE inventory_items (
    ingredient_name TEXT PRIMARY KEY,
    ingredient_unit_name TEXT NOT NULL,
    ingredient_unit_quantity DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (ingredient_name) REFERENCES ingredients (name),
    FOREIGN KEY (ingredient_unit_name) REFERENCES units (name)
);

CREATE TABLE store_items (
    name TEXT PRIMARY KEY,
    ingredient_name TEXT NOT NULL,
    ingredient_unit_name TEXT NOT NULL,
    ingredient_unit_quantity DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION,
    FOREIGN KEY (ingredient_name) REFERENCES ingredients (name),
    FOREIGN KEY (ingredient_unit_name) REFERENCES units (name)
);

CREATE TABLE unit_conversions (
    from_unit_name TEXT NOT NULL,
    from_unit_quantity DOUBLE PRECISION NOT NULL,
    to_unit_name TEXT NOT NULL,
    to_unit_quantity DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (from_unit_name, to_unit_name),
    FOREIGN KEY (from_unit_name) REFERENCES units (name),
    FOREIGN KEY (to_unit_name) REFERENCES units (name)
);

CREATE TABLE dimension_conversions (
    ingredient_name TEXT NOT NULL,
    from_unit_name TEXT NOT NULL,
    from_unit_quantity DOUBLE PRECISION NOT NULL,
    to_unit_name TEXT NOT NULL,
    to_unit_quantity DOUBLE PRECISION NOT NULL,
    PRIMARY KEY (ingredient_name, from_unit_name, to_unit_name),
    FOREIGN KEY (from_unit_name) REFERENCES units (name),
    FOREIGN KEY (to_unit_name) REFERENCES units (name)
);

INSERT INTO dimensions (name) VALUES ('unit');
INSERT INTO dimensions (name) VALUES ('volume');
INSERT INTO dimensions (name) VALUES ('weight');

INSERT INTO units (name, dimension_name) VALUES ('', 'unit');
INSERT INTO units (name, dimension_name) VALUES ('cup', 'volume');
INSERT INTO units (name, dimension_name) VALUES ('tablespoon', 'volume');
INSERT INTO units (name, dimension_name) VALUES ('ounce', 'weight');
INSERT INTO units (name, dimension_name) VALUES ('gram', 'weight');
INSERT INTO units (name, dimension_name) VALUES ('pound', 'weight');

UPDATE dimensions SET base_unit_name = 'cup' WHERE name = 'volume';
UPDATE dimensions SET base_unit_name = 'gram' WHERE name = 'weight';

INSERT INTO ingredients (name) VALUES ('egg');
INSERT INTO ingredients (name) VALUES ('cheese');
INSERT INTO ingredients (name) VALUES ('chopped peppers');
INSERT INTO ingredients (name) VALUES ('sugar');

INSERT INTO recipes (name, servings) VALUES ('spicy omelette', 1);
INSERT INTO recipes (name, servings) VALUES ('omelette du fromage', 1);

INSERT INTO recipe_ingredients (recipe_name, ingredient_name, ingredient_unit_name, ingredient_unit_quantity) VALUES ('spicy omelette', 'egg', '', 2);
INSERT INTO recipe_ingredients (recipe_name, ingredient_name, ingredient_unit_name, ingredient_unit_quantity) VALUES ('spicy omelette', 'cheese', 'cup', 0.25);
INSERT INTO recipe_ingredients (recipe_name, ingredient_name, ingredient_unit_name, ingredient_unit_quantity) VALUES ('spicy omelette', 'chopped peppers', 'cup', 0.25);

INSERT INTO recipe_ingredients (recipe_name, ingredient_name, ingredient_unit_name, ingredient_unit_quantity) VALUES ('omelette du fromage', 'egg', '', 2);
INSERT INTO recipe_ingredients (recipe_name, ingredient_name, ingredient_unit_name, ingredient_unit_quantity) VALUES ('omelette du fromage', 'cheese', 'cup', 0.5);

INSERT INTO unit_conversions (from_unit_name, from_unit_quantity, to_unit_name, to_unit_quantity) VALUES ('cup', 1, 'tablespoon', 16);
INSERT INTO unit_conversions (from_unit_name, from_unit_quantity, to_unit_name, to_unit_quantity) VALUES ('pound', 1, 'ounce', 16);
INSERT INTO unit_conversions (from_unit_name, from_unit_quantity, to_unit_name, to_unit_quantity) VALUES ('gram', 1, 'ounce', 0.035274);

INSERT INTO dimension_conversions (ingredient_name, from_unit_name, from_unit_quantity, to_unit_name, to_unit_quantity) VALUES ('sugar', 'cup', 1, 'gram', 200);