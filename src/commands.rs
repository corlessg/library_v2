use bson::Document;
use reqwest::{StatusCode,Response};
use serde_json::Value;
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};


use crate::models::Book;

// To be used in things...

// Function to call the Open Library API returning an error message if it fails to find the book and the response text if it does find the book
pub async fn call_openlibrary_api(isbn: &String) -> Result<Response, reqwest::Error> {

    // let url: String = "https://openlibrary.org/isbn/".to_owned() + isbn + ".json".to_owned();
    let url: String = format!("https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=data&format=json",isbn);
    
    // Use the reqwest library to send a GET request to the API endpoint
    reqwest::get(url).await
    
}
pub async fn book_details_api(isbn: String) -> Result<Response,String> {

    let res = call_openlibrary_api(&isbn).await;
    match res {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => Ok(response),
                StatusCode::NOT_FOUND => Err("Book was not found".to_string()),
                _ => Err(format!("Unexpected status code: {}",response.status())),
            }
        }
        Err(err) => Err(format!("Error calling OpenLibrary API: {}",err)),
    }
} 

pub async fn load_mongo_client() -> Client {
    // Load the MongoDB connection string from an environment variable:
    // let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_uri = "mongodb://localhost:27017/";

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await;
    Client::with_options(options.unwrap()).unwrap()
}


pub fn modify_json_structure(json_str: &str) -> Result<String,String> {
    let mut json_value: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");

    // Extract the inner JSON object and its key
    if let Some((isbn_key, inner_json)) = json_value.as_object_mut().and_then(|obj| obj.iter_mut().next()) {

        // Add the attribute of ID to be the unique identifier for MongoDB using the ISBN
        let id = isbn_key.split(":").last().unwrap();
        inner_json["_id"] = Value::String(id.to_string());

        //Parse into book Struct
        let book: Book = serde_json::from_value(inner_json.clone()).expect("Failed to parse into Book struct");
        
        Ok(serde_json::to_string_pretty(&book).expect("Failed to serialize final JSON"))
    }
    else {
        Err(format!("Error calling OpenLibrary API"))
    }
}

pub fn json_to_bson(json_str: &str) -> Result<Document, serde_json::Error> {
    // Parse JSON into serde_json::Value
    let json_value: Value = serde_json::from_str(json_str)?;
    
    // Convert serde_json::Value into BSON Document
    let bson_document = bson::to_document(&json_value);

    Ok(bson_document.expect("Couldn't convert json to bson"))
}
