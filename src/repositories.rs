// holds all of the CRUD operations with MongoDB
use mongodb::{error::Error, options::{ClientOptions, ResolverConfig}, results::DeleteResult, Client, Collection};
use bson::{Document,doc};

use crate::{commands, models::Book};
use crate::library::DbConn;

pub struct LibraryRespository;

impl LibraryRespository {
    
    pub async fn find_book_isbn(isbn: String) -> Result<Option<Document>,Error> {
        let client = commands::load_mongo_client().await;
        let books = client.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by ISBNQWE
        let result = books.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn delete_book_isbn(isbn: String) -> Result<DeleteResult,Error> {
        let client = commands::load_mongo_client().await;
        let books: Collection<Book> = client.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by ISBN
        let result = books.delete_one(filter, None).await?;

        Ok(result)
    }

    pub async fn create_book(isbn: String) {
        let details = commands::book_details_api(isbn).await.unwrap().text().await.unwrap();
    
        // this fixes the ISBN number name to be _id which is the unique identifier field automatically used by mongoDB
        let details_edited = details.replacen("ISBN", "_id", 1);
        
        let details_edited_flat = commands::modify_json_structure(details_edited.as_str());
     
        let new_doc = commands::json_to_bson(details_edited_flat.as_ref().unwrap().as_str()).unwrap();
    
        let client = commands::load_mongo_client().await;
        let books = client.database("library").collection("books");
    
        match books.insert_one(new_doc, None).await {
            Ok(insert_result) => {
                println!("New document ID: {}", insert_result.inserted_id);
            }
            Err(err) => {
                println!("Error occurred {}",err);
            }
        }  
    
    }



}