// models lists all necessary structs we expect to need in our database
// these will be derived from what is returned calling the ISBN API

use serde::{Deserialize,Serialize};


#[allow(warnings)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    _id: String,
    url: Option<String>,
    key: Option<String>,
    title: String,
    subtitle: Option<String>,
    authors: Vec<Author>,
    number_of_pages: Option<u32>,
    publish_date: Option<String>,
    subjects: Option<Vec<Subject>>,
    #[serde(default)]
    location: Location,
    // excerpts: Option<Vec<Excerpt>>,
    // weight: Option<String>,
    // identifiers: Option<Identifiers>,
    // classifications: Option<Classifications>,
    // publishers: Option<Vec<Publisher>>,
    // ebooks: Option<Vec<Ebook>>,
    // cover: Option<Cover>,b
}

// impl Default for Book {
//     fn default() -> Self {
//         Self {
//             _id: String::new(),
//             url: None,
//             key: None,
//             title: String::from("Untitled"),
//             subtitle: None,
//             authors: Vec::new(),
//             number_of_pages: None,
//             publish_date: None,
//             subjects: None,
//             #[serde(default)]
//             location: Location::default(), // Explicitly use the default value for Location
//         }
//     }
// }

#[allow(warnings)]
#[derive(Debug,Serialize, Deserialize)]
struct Author {
    // url: Option<String>,
    name: Option<String>,
}

#[allow(warnings)]
#[derive(Debug,Serialize, Deserialize)]
struct Subject {
    name: Option<String>,
    // url: Option<String>,
}

#[allow(warnings)]
#[derive(Debug,Serialize, Deserialize)]
pub struct Location {
    pub house: String,
    pub room: String,
    pub owner: String
}

impl Default for Location {
    fn default() -> Self {
        Self {
            house: "Mars".to_string(),
            room: "Library".to_string(),
            owner: "Garrett".to_string()
        }
    }
}

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Identifiers {
//     goodreads: Option<Vec<String>>,
//     librarything: Option<Vec<String>>,
//     isbn_10: Option<Vec<String>>,
//     isbn_13: Option<Vec<String>>,
//     lccn: Option<Vec<String>>,
//     openlibrary: Option<Vec<String>>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Classifications {
//     lc_classifications: Option<Vec<String>>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Publisher {
//     name: Option<String>,
// }


// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Excerpt {
//     text: Option<String>,
//     comment: Option<String>,
//     first_sentence: Option<bool>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Ebook {
//     preview_url: Option<String>,
//     availability: Option<String>,
//     formats: Option<Formats>,
//     read_url: Option<String>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Formats {
//     pdf: Option<FormatUrl>,
//     epub: Option<FormatUrl>,
//     text: Option<FormatUrl>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct FormatUrl {
//     url: Option<String>,
// }

// #[allow(warnings)]
// #[derive(Debug, Deserialize)]
// struct Cover {
//     small: Option<String>,
//     medium: Option<String>,
//     large: Option<String>,
// }