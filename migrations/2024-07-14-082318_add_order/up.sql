CREATE TABLE products (
    product_id UUID PRIMARY KEY,
    product_name VARCHAR(100) NOT NULL UNIQUE,
    product_description TEXT NOT NULL,
    numerator INT NOT NULL,
    denomenator INT NOT NULL
);

CREATE TABLE orders (
    order_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    order_date TIMESTAMP NOT NULL,
    total_amount_numerator INT NOT NULL,
    total_amount_denomenator INT NOT NULL,
    currency TEXT NOT NULL,
    order_status TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id)
);

CREATE TABLE order_item (
    order_item_id UUID PRIMARY KEY,
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INT NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id)
);