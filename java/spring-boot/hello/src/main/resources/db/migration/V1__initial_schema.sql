CREATE TABLE foods (
    id serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    calories INTEGER,
    barcode VARCHAR(13) NOT NULL
 );
