
use mongodb::{error::Error, results::DeleteResult, Client};

use crate::repositories::LibraryRespository;



async fn load_mongo_client() -> Client {
    // Load the MongoDB connection string from an environment variable:
    // let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    Client::with_uri_str("mongodb://user:pass@mongodb/library?authSource=admin").await.expect("Failure to connect to MongoDB client")
}

pub async fn find_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book = LibraryRespository::find_book_isbn(&mut c, &isbn).await;

    //TODO! if None is returned, we need to alert instead of saying found None!
    match book {
        Ok(book) => if book.is_none() {
            println!("Book {:?} not found!", isbn)
        } else {
            println!("Successfully found the book! \n {:?} ", book)
        },
        Err(book) => println!("Could not find the book to the library due to: {:?} ", book)
    }
}

pub async fn add_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book = LibraryRespository::create_book(&mut c, isbn).await;

    // TODO make sure we tell the user what book they just input into the system!

    match book {
        Ok(book) => println!("Successfully added the book: {:?}! ", book),
        Err(book) => println!("Could not add the book to the library due to: {:?} ", book)
    }

}

pub async fn remove_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book: Result<DeleteResult, Error> = LibraryRespository::delete_book_isbn(&mut c, &isbn).await;

    //TODO! Parse the result if deleted_count == 0, then it didn't find the result!
    
    match book {
        Ok(book) => 
            if book.deleted_count == 0 {
                println!("Book {:?} not found!", isbn)
            } else {
                println!("Successfully removed the book: {:?}! ", book)
            }

        Err(book) => println!("Could not remove the book to the library due to: {:?} ", book)
    }
}

pub async fn update_book(isbn: String, ) {

}

