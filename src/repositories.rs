// holds all of the CRUD operations with MongoDB
use mongodb::{error::Error, options::{ClientOptions, ResolverConfig}, results::{DeleteResult, InsertOneResult}, Client, Collection, Cursor};
use bson::{Document,doc};
use rocket::futures::TryStreamExt;
use rocket_db_pools::Connection;

use crate::{commands, models::Book};
use crate::rocket_routes::DbConn;

pub struct LibraryRespository;


impl LibraryRespository {
    
    pub async fn find_book_isbn(c: &mut Connection<DbConn>, isbn: String) -> Result<Option<Document>,Error> {
        
        let books = c.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by ISBNQWE
        let result = books.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn delete_book_isbn(c: &mut Connection<DbConn>, isbn: String) -> Result<DeleteResult,Error> {
        let books: Collection<Book> = c.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by ISBN
        let result = books.delete_one(filter, None).await?;

        Ok(result)
    }

    pub async fn create_book(c: &mut Connection<DbConn>,isbn: String) -> Result<InsertOneResult,Error> {
        let details = commands::book_details_api(isbn).await.unwrap().text().await.unwrap();
    
        // this fixes the ISBN number name to be _id which is the unique identifier field automatically used by mongoDB
        let details_edited = details.replacen("ISBN", "_id", 1);
        
        let details_edited_flat = commands::modify_json_structure(details_edited.as_str());
     
        let new_doc = commands::json_to_bson(details_edited_flat.as_ref().unwrap().as_str()).unwrap();
    
        let books = c.database("library").collection("books");
    
        books.insert_one(new_doc, None).await
        
    }

    pub async fn find_random_books(c: &mut Connection<DbConn>) -> Result<Vec<Document>, Error> {
        let search = doc! { "$sample": { "size": 5 } };

        let books: Collection<Document> = c.database("library").collection("books");
        
        let mut random_books = books.aggregate([search],None).await?;

        let mut result: Vec<Document> = Vec::new();

        while let Some(doc) = random_books.try_next().await? {
            result.push(doc);
        }
        
        Ok(result)

    }



}