extern crate library;

use rocket;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            library::rocket_routes::books::get_book_by_isbn,
            library::rocket_routes::books::create_book,
            library::rocket_routes::books::delete_book,
            library::rocket_routes::books::get_books,
            library::rocket_routes::books::update_book
            ])
        .attach(library::rocket_routes::DbConn::init())
        .launch()
        .await;
}