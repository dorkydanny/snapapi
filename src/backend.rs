use actix_web::{get, post, web::{Path, Query}, HttpResponse};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl};
use uuid::Uuid;
use crate::{db::establish_connection, models::User as UserStruct, schema::*};
use crate::models::*;
use crate::models::OrderStruct;
use crate::schema; 
use crate::schema::users::dsl::*;

#[allow(unused_imports)] //actually neccessary, compiler does not understand as not referenced directly.
use serde::{Serialize, Deserialize}; // required by Uuid and to Deserialize the Structs

#[get("/api/v1/{user_id}")]
pub async fn user_details(input_user_id: Path<Uuid>) -> HttpResponse {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    println!("{}", input_user_id);
    let results = users
        .filter(user_id.eq(input_user_id.into_inner()))
        .filter(removed.eq(false)) 
        .load::<User>(&mut connection);
    match results  {
            Ok(user) => HttpResponse::Ok().json(user), // Respond with user details if found
            Err(_) => HttpResponse::NotFound().body("USER NOT FOUND OR DATABASE ERROR"), // Return 404 if user not found or other error
    }
    
}

    #[get("/api/v1/{user_id}/history")]
    pub async fn user_purchases(input_user_id: Path<Uuid>) -> HttpResponse {
        use crate::schema::orders::dsl::*;
        let mut connection = establish_connection();
        let results = schema::orders::table
            .filter(schema::orders::user_id.eq(input_user_id.into_inner()))
            .load::<OrderStruct>(&mut connection);
        match results  {
                Ok(purchase) => HttpResponse::Ok().json(purchase), // Respond with purchase details if found
                Err(_) => HttpResponse::NotFound().body("USER NOT FOUND OR DATABASE ERROR"), // Return 404 if user not found or other error
        }
        
    }

#[post("/api/v1/new_user")]
pub async fn create_user(user_data: Query<UserStruct>) -> HttpResponse {
    let mut connection = establish_connection();
    let new_user = user_data.into_inner();
    let diesel_status = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection);
    match diesel_status {
        Ok(_) => return HttpResponse::Ok().body(new_user.user_id.to_string()),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string())
    }
}

#[post("/api/v1/new_product")]
pub async fn create_product(product_data: Query<Product>) -> HttpResponse {
    let mut connection = establish_connection();
    let new_product: Product = product_data.into_inner();
    let diesel_status = diesel::insert_into(products::table)
        .values(&new_product)
        .execute(&mut connection);
    match diesel_status {
        Ok(_) => return HttpResponse::Ok().body(new_product.product_id.to_string()),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string())
    }
}

#[post("/api/v1/new_order")]
pub async fn create_order(order_data: Query<OrderStruct>) -> HttpResponse {
    let mut connection = establish_connection();
    let new_order: OrderStruct = order_data.into_inner();
    let diesel_status = diesel::insert_into(orders::table)
        .values(&new_order)
        .execute(&mut connection);
    match diesel_status {
        Ok(_) => return HttpResponse::Ok().body(new_order.order_id.to_string()),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string())
    }
}

#[post("/api/v1/new_order_item")]
pub async fn create_order_item(order_item_data: Query<OrderItem>) -> HttpResponse {
    let mut connection = establish_connection();
    let new_order_item: OrderItem = order_item_data.into_inner();
    let diesel_status = diesel::insert_into(order_item::table)
        .values(&new_order_item)
        .execute(&mut connection);
    match diesel_status {
        Ok(_) => return HttpResponse::Ok().body(new_order_item.order_id.to_string()),
        Err(e) => return HttpResponse::BadRequest().body(e.to_string())
    }
}