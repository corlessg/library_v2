use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::rocket_routes::{DbConn,server_error};
use crate::repositories::LibraryRespository;
use crate::models::Location;

#[rocket::get("/books/<id>")]
pub async fn get_book_by_isbn(mut db: Connection<DbConn>, id: String) -> Result<Value,Custom<Value>>{
    LibraryRespository::find_book_isbn(&mut db, &id).await
    .map(|a_book| json!(a_book))
    .map_err(|e| server_error(e.into()))
}

#[rocket::get("/books")]
pub async fn get_books(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>>{
    LibraryRespository::find_random_books(&mut db).await
    .map(|rand_books| json!(rand_books))
    .map_err(|e| server_error(e.into()))
}

#[rocket::post("/books", data="<book_isbn>")]
pub async fn create_book(mut db: Connection<DbConn>, book_isbn: String) -> Result<Custom<Value>,Custom<Value>> {
    LibraryRespository::create_book(&mut db, book_isbn).await
    .map(|a_book| Custom(Status::Created, json!(a_book)))
    .map_err(|e| server_error(e.into()))
}

#[rocket::put("/books/<id>",format="json", data="<book_location>")]
pub async fn update_book(mut db: Connection<DbConn>, id: String, book_location: Json<Location>) -> Result<Value, Custom<Value>> {
    let book_loc = book_location.into_inner();
    
    LibraryRespository::update_book_location(&mut db, id, book_loc).await
        .map(|a_loc| json!(a_loc))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/books", data="<book_isbn>")]
pub async fn delete_book(mut db: Connection<DbConn>, book_isbn: String) -> Result<Custom<Value>,Custom<Value>> {
    LibraryRespository::delete_book_isbn(&mut db, &book_isbn).await
    .map(|a_book| Custom(Status::Created, json!(a_book)))
    .map_err(|e| server_error(e.into()))
}