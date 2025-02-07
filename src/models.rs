// models lists all necessary structs we expect to need in our database
// these will be derived from what is returned calling the ISBN API

use core::fmt;

use serde::{Deserialize,Serialize};

// TODO GPC in theory, you should add this enumeration to all locations... 
#[derive(Debug, Serialize, Deserialize)]
pub enum HouseLocations {
    Mars,
    Bethany
}

impl fmt::Display for HouseLocations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let house_loc = match self {
            HouseLocations::Bethany => "Bethany",
            HouseLocations::Mars => "Mars"
        };
        write!(f, "{}", house_loc)
    }
}


#[derive(Debug,Serialize, Deserialize)]
pub enum CheckedStatus {
    CheckedIn,
    CheckedOut
}

impl Default for CheckedStatus {
    fn default() -> Self {
        CheckedStatus::CheckedIn
    }
}

#[allow(warnings)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    _id: String,
    url: Option<String>,
    key: Option<String>,
    pub title: String,
    subtitle: Option<String>,
    authors: Option<Vec<Author>>,
    number_of_pages: Option<u32>,
    publish_date: Option<String>,
    subjects: Option<Vec<Subject>>,
    #[serde(default)]
    location: Location,
    #[serde(default)]
    checked_status: CheckedStatus,
    borrower: Option<String>
    // excerpts: Option<Vec<Excerpt>>,
    // weight: Option<String>,
    // identifiers: Option<Identifiers>,
    // classifications: Option<Classifications>,
    // publishers: Option<Vec<Publisher>>,
    // ebooks: Option<Vec<Ebook>>,
    // cover: Option<Cover>,b
}


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

// TODO: In thoery this should be address, state, etc. but not putting that onto Github...
#[allow(warnings)]
#[derive(Debug,Serialize, Deserialize)]
pub struct Location {
    pub house: HouseLocations,
    pub room: String,
    pub owner: String
}

impl Default for Location {
    fn default() -> Self {
        Self {
            house: HouseLocations::Mars,
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