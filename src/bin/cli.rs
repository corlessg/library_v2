
use clap::{Command, Arg};
use serde_json::{Map, Value};

extern crate library;
use library::commands;

#[tokio::main]
async fn main() {
    let matches = Command::new("Library")
    .about("Library Commands")
    .arg_required_else_help(true)
    .subcommand(
        Command::new("search")
            .about("Search for a book")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("isbn")
                    .about("Search by ISBN")
                    .arg_required_else_help(true)
                    .arg(Arg::new("isbn").required(true))

            )
            .subcommand(
                Command::new("title")
                .about("Search by title")
                .arg_required_else_help(true)
                .arg(Arg::new("title").required(true))
            )
        )
    .subcommand(
        Command::new("add")
            .about("Add book to library")
            .arg_required_else_help(true)
            .arg(Arg::new("isbn").required(true))
    )
    .subcommand(
        Command::new("remove")
            .about("Remove book from library by ISBN")
            .arg(Arg::new("isbn").required(true))
    )
    .subcommand(
        Command::new("update")
            .about("Update book's location from library by ISBN")
            .arg(Arg::new("isbn")
                .help("The ISBN of the book")
                .required(true))
            .arg(Arg::new("house")
                .help("House location where the book is stored")
                .required(false))
            .arg(Arg::new("room")
                .help("Room within the house")
                .required(false))
            .arg(Arg::new("owner")
                .help("Owner of the book")
                .required(false))
    )
    .subcommand(
        Command::new("scanner")
            .about("Initialize scanner mode: Enter ISBN numbers for rapid adding to database")
    )
    .subcommand(
        Command::new("batch")
            .about("Batch upload from csv file into database")
            .arg_required_else_help(true)
            .arg(Arg::new("file_path").required(true))

    )
    .get_matches();

match matches.subcommand() {
    Some(("search", search_matches)) => match search_matches.subcommand() {
        Some(("isbn",search_matches)) =>
        println!("{:?}",
        commands::find_book(
            search_matches.get_one::<String>("isbn").unwrap().to_owned()
        ).await
        ),
        Some(("title",search_matches)) => 
        println!("{:?}",
        commands::find_books_title(
            search_matches.get_one::<String>("title").unwrap().to_owned()
        ).await
        ),
        _ => {}
    },
    // TODO: Create a way to store location of book
    Some(("add", sub_matches)) => commands::add_book(
        sub_matches.get_one::<String>("isbn").expect("Could not find the string").to_string()
        ).await,
    Some(("update", update_matches)) => {
        let mut update_map = Map::new();

        // For each argument, if it exists, add it to the JSON map.
        if let Some(isbn) = update_matches.get_one::<String>("isbn") {
            update_map.insert("isbn".to_string(), Value::String(isbn.to_string()));
        }
        if let Some(house) = update_matches.get_one::<String>("house") {
            update_map.insert("house".to_string(), Value::String(house.to_string()));
        }
        if let Some(room) = update_matches.get_one::<String>("room") {
            update_map.insert("room".to_string(), Value::String(room.to_string()));
        }
        if let Some(owner) = update_matches.get_one::<String>("owner") {
            update_map.insert("owner".to_string(), Value::String(owner.to_string()));
        }

        // Confident in isbn existing as it is a required field for CLI call
        let isbn = update_map.get("isbn").unwrap().to_string();
        // Convert the map into a JSON Value.
        let location_json = Value::Object(update_map);
        
        commands::update_book(isbn,location_json).await
    }
    Some(("remove", sub_matches)) => commands::remove_book(
        sub_matches.get_one::<String>("isbn").expect("Could not parse the string inout").to_string()
    ).await,
    Some(("scanner", _)) => library::scanner::scan_books().await,
    Some(("batch", sub_matches)) => commands::batch_upload(
        sub_matches.get_one::<String>("file_path").expect("Could not find the filepath")
    ).await,
    _ => {}            
}
}

