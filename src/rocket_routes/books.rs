use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Value,json};
use rocket_db_pools::Connection;

use crate::rocket_routes::{DbConn,server_error};
use crate::repositories::LibraryRespository;

#[rocket::get("/books/<id>")]
pub async fn get_books(mut db: Connection<DbConn>, id: String) -> Result<Value,Custom<Value>>{
    LibraryRespository::find_book_isbn(&mut db, id).await
    .map(|a_book| json!(a_book))
    .map_err(|e| server_error(e.into()))
}

#[rocket::post("/books", data="<book_isbn>")]
pub async fn create_book(mut db: Connection<DbConn>, book_isbn: String) -> Result<Custom<Value>,Custom<Value>> {
    LibraryRespository::create_book(&mut db, book_isbn).await
    .map(|a_book| Custom(Status::Created, json!(a_book)))
    .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/books", data="<book_isbn>")]
pub async fn delete_book(mut db: Connection<DbConn>, book_isbn: String) -> Result<Custom<Value>,Custom<Value>> {
    LibraryRespository::delete_book_isbn(&mut db, book_isbn).await
    .map(|a_book| Custom(Status::Created, json!(a_book)))
    .map_err(|e| server_error(e.into()))
}