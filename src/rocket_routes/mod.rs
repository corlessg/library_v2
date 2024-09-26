pub mod books;

use std::error::Error;

use rocket::{http::Status, response::status::Custom,serde::json::json};
use rocket_db_pools::Database;
use serde_json::Value;

#[derive(Database)]
#[database("mongodb")]
pub struct DbConn(rocket_db_pools::mongodb::Client);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}",e);
    Custom(Status::InternalServerError,json!("Error"))
}