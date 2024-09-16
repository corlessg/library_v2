extern crate library;

use repositories:

use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("mongodb")]
struct DbConn(rocket_db_pools::mongodb::Client);


#[rocket::get("/books")]
fn get_books(db: Connection<DbConn>) {
    library::
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            get_books
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}