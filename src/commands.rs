
use std::{fs::File, io::BufReader, path::Path};
use csv::ReaderBuilder;
use mongodb::{error::Error, results::{DeleteResult, UpdateResult}, Client};

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

pub async fn find_books_title(isbn: String) {
    let mut c = load_mongo_client().await;

    let results = LibraryRespository::find_book_title(&mut c, &isbn).await;

    //TODO! if None is returned, we need to alert instead of saying found None!
    match results {
        Ok(results) => { 
            for book in results {
                println!("Book {} found!", book);
            }
        },
        Err(book) => println!("Could not find the book to the library due to: {:?} ", book)
    }
}

pub async fn add_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book = LibraryRespository::create_book(&mut c, isbn).await;

    // TODO make sure we tell the user what book they just input into the system!

    match book {
        Ok((_,book_name)) => println!("Successfully added the book: {:?}! ", book_name),
        Err(e) => println!("Could not add the book to the library due to: {:?} ", e)
    }

}

pub async fn remove_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book = LibraryRespository::delete_book_isbn(&mut c, &isbn).await;
    
    match book {
        Ok((book,book_name)) => 
            if book.deleted_count == 0 {
                println!("Book {:?} not found!", book_name)
            } else {
                println!("Successfully removed the book: {:?}! ", book_name)
            }

        Err(e) => println!("Could not remove the book to the library due to: {:?} ", e)
    }
}

// TODO GPC
// pub async fn update_book(isbn: String, ) {

// }

pub async fn checkout_book(isbn: String, borrower: String) {
    let mut c = load_mongo_client().await;    
    
    let book_update: Result<(UpdateResult,String), Error> = LibraryRespository::checkout_book(&mut c, &isbn, borrower).await;

    match book_update {
        Ok((update, book_name)) => 
            if update.matched_count == 0 {
                println!("book_name {:?} not found!", &isbn)
            } else {
                println!("Successfully checkedout the book: {:?}! ", book_name)
            }

        Err(err) => println!("Could not checkout the book from the library due to: {:?} ", err.get_custom::<String>()
    .expect("Problem parsing custom error"))
    }
}

pub async fn checkin_book(isbn: String) {
    let mut c = load_mongo_client().await;

    let book: Result<(UpdateResult, String), Error> = LibraryRespository::checkin_book(&mut c, &isbn).await;
    
    match book {
        Ok((book,book_name)) => {
            if book.matched_count == 0 {
                println!("Book {:?} not found!", book_name)
            } else {
                println!("Successfully checked in the book: {:?}! ", book_name)
            }
        },
        Err(book) => println!("Could not check the book into the library due to: {:?} ", book.get_custom::<String>()
        .expect("Problem parsing custom error"))
    }
}

pub async fn batch_upload(file_path: &str) {
    let mut c = load_mongo_client().await;

    // This should scale better and be derived from an enum
    // println!("Where are these books stored? (Mars or Bethany)");
// 
    // let input = read_user_input().trim().to_lowercase();

    // let loc = Location {
    //     house: input.to_string(),
    //     room: "library".to_string(),
    //     owner: "Garrett".to_string()
    // };

    //read file path
    let path = Path::new(file_path);

    if path.extension().and_then(|s| s.to_str()) != Some("csv") {
        return println!("Provided file is not a CSV file.")
    }

    let file = File::open(file_path);
    println!("{:?}",file);

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut reader = ReaderBuilder::new().has_headers(false).from_reader(reader);


            for result in reader.records() {
                if let Ok(record) = result {
                    let isbn = record.get(0).expect("error parsing line");
                    let book_search = LibraryRespository::find_book_isbn(&mut c, &isbn.to_string()).await;
                    
                    match book_search {
                        Ok(book) => if book.is_none() {
                            println!("Adding: {}",isbn);
                            if let Err(_) = LibraryRespository::create_book(&mut c, isbn.to_string()).await {
                                println!("Cannot add: {}.. moving on", isbn);
                            }
                            // if let Err(_) = LibraryRespository::update_book_location(&mut c, isbn.to_string(), loc_tar).await {
                            //     println!("Cannot update {}'s location... moving on",isbn);
                            // }
                        } else {
                            continue
                        },
                        Err(e) => println!("Error while adding book: {}",e)
                    }
                } else {
                    println!("error parsing line")
                }
                
            }
        },
        Err(e) => println!("Error returned: {}",e)
    }
}