CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email TEXT NOT NULL,
    removed BOOLEAN NOT NULL
);

CREATE TABLE products (
    product_id UUID PRIMARY KEY,
    product_name VARCHAR(100) NOT NULL UNIQUE,
    product_description TEXT NOT NULL,
    numerator INT NOT NULL,
    denomenator INT NOT NULL
);

