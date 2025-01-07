
use clap::{Command, Arg};


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
        // Some(("title",search_matches)) => 
        // println!("{:?}",
        // LibraryRespository::::commands::find_book_title(
        //     search_matches.get_one::<String>("title").unwrap().to_owned()
        // ).await
        // ),
        _ => {}
    },
    // TODO: Create a way to store location of book
    Some(("add", sub_matches)) => commands::add_book(
        sub_matches.get_one::<String>("isbn").expect("Could not find the string").to_string()
        ).await,
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

