use diesel::{dsl::Order, prelude::*};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    removed: bool
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct OrderStruct {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub order_date: NaiveDateTime,
    total_amount_numerator: i32,
    total_amount_denomenator: i32,
    currency: String,
    pub order_status: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name =products)]
pub struct Product {
    pub product_id: Uuid,
    product_name: String,
    product_description: String,
    numerator: i32,
    denomenator: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = order_item)]
pub struct OrderItem {
    pub order_item_id: Uuid,
    pub order_id: Uuid,
    product_id: Uuid,
    quantity: i32,
}

/*-----------------------------------*/