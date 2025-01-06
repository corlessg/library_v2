use bson::Document;
use reqwest::{StatusCode,Response};
use serde_json::Value;
use mongodb::{options::{ClientOptions, ResolverConfig}, Client};


use crate::models::Book;

// To be used in things...

// Function to call the Open Library API returning an error message if it fails to find the book and the response text if it does find the book
pub async fn fetch_book_details(isbn: String) -> Result<String, String> {
    let url = format!(
        "https://openlibrary.org/api/books?bibkeys=ISBN:{}&jscmd=data&format=json",
        isbn
    );

    // Send a GET request to the Open Library API
    let response = reqwest::get(&url).await.map_err(|err| format!("Error calling OpenLibrary API: {}", err))?;

    // Check the status code
    match response.status() {
        StatusCode::OK => {
            let text = response.text().await.map_err(|err| format!("Error reading response text: {}", err))?;
            if text.trim() == "{}" {
                Err("Book was not found".to_string())
            } else {
                Ok(text)
            }
        }
        StatusCode::NOT_FOUND => Err("Book was not found".to_string()),
        _ => Err(format!("Unexpected status code: {}", response.status())),
    }
}



pub fn modify_json_structure(json_str: &str) -> Result<String,String> {
    let mut json_value: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");

    // Extract the inner JSON object and its key
    if let Some((isbn_key, inner_json)) = json_value.as_object_mut().and_then(|obj| obj.iter_mut().next()) {

        // Add the attribute of ID to be the unique identifier for MongoDB using the ISBN
        let id = isbn_key.split(":").last();
        match id {
            Some(id) => {
                inner_json["_id"] = Value::String(id.to_string());

                //Parse into book Struct
                let book: Book = serde_json::from_value(inner_json.clone()).expect("Failed to parse into Book struct");
                
                Ok(serde_json::to_string_pretty(&book).expect("Failed to serialize final JSON"))
            },
            _ => Err(format!("could not split the isbn key: {}",isbn_key))
        }
        
    }
    else {
        Err(format!("Error modifying the JSON response: {}",json_str))
    }
}

pub fn json_to_bson(json_str: &str) -> Result<Document, serde_json::Error> {
    // Parse JSON into serde_json::Value
    let json_value: Value = serde_json::from_str(json_str)?;
    
    // Convert serde_json::Value into BSON Document
    let bson_document = bson::to_document(&json_value);

    Ok(bson_document.expect("Couldn't convert json to bson"))
}
