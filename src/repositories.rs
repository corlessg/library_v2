// holds all of the CRUD operations with MongoDB
use mongodb::{error::Error, results::{DeleteResult, InsertOneResult, UpdateResult}, Client, Collection};
use bson::{doc, Bson, Document};
use rocket::futures::TryStreamExt;

use crate::{models::{Book, Location}, utils};

pub struct LibraryRespository;


impl LibraryRespository {
    
    // TODO - why is this returning an option<Doc> rather than a clean doc?
    pub async fn find_book_isbn(c: &mut Client, isbn: &String) -> Result<Option<Document>,Error> {
        
        let books = c.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by ISBNQWE
        let result = books.find_one(filter, None).await?;

        Ok(result)
    }

    pub async fn find_book_title(c: &mut Client, title: &String) -> Result<Vec<Document>,Error> {
        
        let books = c.database("library").collection("books");

        let filter = doc! { "title": title };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();

        // Find the document by TITLE
        let mut books = books.find(filter, None).await?;

        let mut result: Vec<Document> = Vec::new();

        while let Some(doc) = books.try_next().await? {
            result.push(doc);
        }

        Ok(result)
    }

    pub async fn delete_book_isbn(c: &mut Client, isbn: &String) -> Result<(DeleteResult,String),Error> {
        let books: Collection<Book> = c.database("library").collection("books");

        let filter = doc! { "_id": &isbn };
        // Optional: Specify additional options, such as projection or other query options
        // let options = FindOneOptions::builder().build();
        let book = books.find_one(filter.clone(), None).await?;

        let book_name = book.map(|b| b.title.clone())
            .expect(format!("When attempting to delete {} title could not be found to delete",isbn).as_str());
        // Find the document by ISBN
        let result = books.delete_one(filter, None).await?;

        Ok((result,book_name))
    }

    pub async fn create_book(c: &mut Client,isbn: String, house_loc: Option<String>) -> Result<(InsertOneResult,String),Error> {
        let details = utils::fetch_book_details(isbn.clone()).await
        .map_err(|e| Error::custom(format!("API Error {}",e)))?;

        
        // this fixes the ISBN number name to be _id which is the unique identifier field automatically used by mongoDB        
        let details_edited = details.replacen("ISBN", "_id", 1);
        
        // modified the JSON structure
        let details_edited_flat = utils::modify_json_structure(details_edited.as_str())
            .expect("Failed to flatten the JSON details returned by OpenLibrary's API");

        let mut new_doc = utils::json_to_bson(details_edited_flat.as_str())
            .expect("Failed to convert JSON to BSON following the OpenLib API call");
        
        // Inject the house value if it exists
        if let Some(house_loc) = house_loc {
            new_doc.insert("location.house", house_loc);
        }

        let book_name = new_doc.get_str("title")
            .expect(format!("Could not resolve title name from book {}",isbn)
            .as_str())
            .to_string();

        let books = c.database("library").collection("books");
        
        let result = books.insert_one(new_doc, None).await?;

        Ok((result,book_name))
                
    }

    pub async fn update_book_location(c: &mut Client, isbn: &String, loc: Location) -> Result<(UpdateResult,String),Error> {
        let books: Collection<Document> = c.database("library").collection("books");

        let filter = doc! { "_id": isbn };
        
        let book = books.find_one(filter.clone(), None).await?;
        println!("{:?}",book);
        let book_name = book.map(|b| b.get_str("title").expect("'Title' does not exist in document").to_string())
            .expect(format!("Could not resolve book name while updating {}",isbn).as_str());

        let mut new_loc_doc = Document::new();

        new_loc_doc.insert("location.house",Bson::String(loc.house.to_string()));
        new_loc_doc.insert("location.room",Bson::String(loc.room));
        new_loc_doc.insert("location.owner",Bson::String(loc.owner));

        let new_loc = doc! {"$set": new_loc_doc };

        let result = books.update_one(filter, new_loc, None).await?;

        Ok((result,book_name))
    }

    pub async fn checkout_book(c: &mut Client, isbn: &String, borrower: String) -> Result<(UpdateResult,String), Error> {
        let books: Collection<Document> = c.database("library").collection("books");
        // Check to make sure book exists in library

        // Check it's status - and if it is currently checkedout, return error
        // If it is checkedin currently, check it out (need some way to get the user name into the check out category)

        let filter = doc! { "_id": isbn };

        // Prep BSON doc with updated information
        let mut book_update_doc = Document::new();
        book_update_doc.insert("borrower",Bson::String(borrower));
        book_update_doc.insert("checked_status",Bson::String("CheckedOut".to_string()));

        let book_update = doc! {"$set": book_update_doc };

        if let Ok(Some(book)) = books.find_one(filter.clone(), None).await {
            
           let book_name = book.get_str("title");

           match book.get_str("checked_status") {
                Ok(status) if status == "CheckedOut" => {
                    // Use unwrap because high confidence of existing borrower if CheckedOut
                    let borrower = book.get_str("borrower").unwrap_or("Unknown borrower").to_string();
                    return Err(Error::custom(
                        format!("Could not check out book {:?}: Already checked out by {}",
                        book_name.expect("Problem retriving book name"), borrower)))
                },
                Ok(_) => {
                    println!("Book is available.");
                    let result = books.update_one(filter, book_update, None).await?;
                    Ok((result,book_name.expect("problem retrieving book name").to_string()))
                },
                Err(e) => Err(Error::custom(format!("Error {} checking out book: {:?}",e,book_name)))
           }
        } else {
            Err(Error::custom("Book not found in the library."))
        }
   
    }


    pub async fn checkin_book(c: &mut Client, isbn: &String) -> Result<(UpdateResult, String), Error> {
        let books: Collection<Document> = c.database("library").collection("books");
        // Check to make sure book exists in library

        // Check it's status - and if it is currently checkedout, return error
        // If it is checkedin currently, check it out (need some way to get the user name into the check out category)

        let filter = doc! { "_id": isbn };

        // Prep BSON doc with updated information
        let mut book_update_doc = Document::new();
        book_update_doc.insert("checked_status",Bson::String("CheckedIn".to_string()));
        book_update_doc.insert("borrower",Bson::String("".to_string()));
        let book_update = doc! {"$set": book_update_doc };

        if let Ok(Some(book)) = books.find_one(filter.clone(), None).await {
            
           let book_name = book.get_str("title");

           match book.get_str("checked_status") {
                Ok(status) if status == "CheckedIn" => {
                    return Err(Error::custom(format!("Book {:?} has already been checked in",book_name.expect("Couldn't find book title"))))
                },
                Ok(_) => {
                    println!("Book is available.");
                    let update = books.update_one(filter, book_update, None).await?;

                    Ok((update,book_name.expect("could not identify the book").to_string()))
                },
                Err(e) => Err(Error::custom(format!("Error {} checking in book: {:?}",e,book_name)))
           }
        } else {
            Err(Error::custom("Book not found in the library."))
        }
        
    }

    
    pub async fn find_random_books(c: &mut Client) -> Result<Vec<Document>, Error> {
        let search = doc! { "$sample": { "size": 5 } };

        let books: Collection<Document> = c.database("library").collection("books");
        
        let mut random_books = books.aggregate([search],None).await?;

        let mut result: Vec<Document> = Vec::new();

        while let Some(doc) = random_books.try_next().await? {
            result.push(doc);
        }
        
        Ok(result)

    }

    pub async fn find_checked_out_books(c: &mut Client) -> Result<Vec<Document>,Error> {
        let books: Collection<Document> = c.database("library").collection("books");
        let query = doc! {"checked_status":"CheckedOut"};
        
        let mut book_query = books.find(query, None).await?;
        
        let mut result: Vec<Document> = Vec::new();

        while let Some(book) = book_query.try_next().await? {
            result.push(book);
        }
        Ok(result)
    }



}