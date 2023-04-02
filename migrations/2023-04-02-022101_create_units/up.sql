CREATE TABLE units (
    name TEXT PRIMARY KEY NOT NULL,
    dimension_name TEXT NOT NULL,
    FOREIGN KEY (dimension_name) REFERENCES dimensions (name)
);