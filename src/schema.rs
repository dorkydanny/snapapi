// @generated automatically by Diesel CLI.

diesel::table! {
    order_item (order_item_id) {
        order_item_id -> Uuid,
        order_id -> Uuid,
        product_id -> Uuid,
        quantity -> Int4,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Uuid,
        user_id -> Uuid,
        order_date -> Timestamp,
        total_amount_numerator -> Int4,
        total_amount_denomenator -> Int4,
        currency -> Text,
        order_status -> Text,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Uuid,
        #[max_length = 100]
        product_name -> Varchar,
        product_description -> Text,
        numerator -> Int4,
        denomenator -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        email -> Text,
        removed -> Bool,
    }
}

diesel::joinable!(order_item -> orders (order_id));
diesel::joinable!(order_item -> products (product_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    order_item,
    orders,
    products,
    users,
);
