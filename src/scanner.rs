use std::io;
use crate::commands;

// TODO GPC - make the return values say the names of the books - follow pattern for the checkin process!!
// TODO GPC - add the locations when adding book(s) batch and single


pub async fn scan_books() {
    print!("\x1B[2J\x1B[1;1H");
    loop {
        println!("Query, add, checkin, or checkout books? (or type 'exit' to quit the program at any time):");

        let action = read_user_input().trim().to_lowercase();

        match action.as_str() {
            "exit" => {
                println!("Exiting the program.");
                std::process::exit(0);
            }
            "query" => query_books().await,
            "checkin" => checkin_books().await,
            "checkout" => checkout_books().await,
            "add" => add_books().await,
            _ => println!("You did not choose an acceptable action."),
        }
    }
}

async fn query_books() {
    loop {
        println!("Enter an ISBN (or type 'return' to return to the main menu");
        let input = read_user_input().trim().to_lowercase();

        if input == "return" {
            break;
        } else if input == "exit" {
            println!("Exiting the program.");
            std::process::exit(0);
        }

        match input.parse::<i64>() {
            Ok(isbn) => {
                commands::find_book(isbn.to_string()).await;
            }
            Err(_) => println!("Invalid input. Please enter a valid ISBN number, 'return', or 'exit'."),
        }
    }
}

async fn checkin_books() {
    loop {
        println!("Enter an ISBN (or type 'return' to return to the main menu):");
        let input = read_user_input().trim().to_lowercase();

        if input == "return" {
            break;
        } else if input == "exit" {
            println!("Exiting the program.");
            std::process::exit(0);
        }

        match input.parse::<i64>() {
            Ok(isbn) => {
                commands::checkin_book(isbn.to_string()).await;
            }
            Err(_) => println!("Invalid input. Please enter a valid ISBN number, 'return', or 'exit'."),
        }
    }
}

async fn checkout_books() {
    loop {
        println!("Enter an ISBN (or type 'return' to return to the main menu:");
        let isbn_input = read_user_input().trim().to_lowercase();

        if isbn_input == "return" {
            break;
        } else if isbn_input == "exit" {
            println!("Exiting the program.");
            std::process::exit(0);
        }

        println!("Enter borrower's name:");
        let borrower_name = read_user_input().trim().to_string();

        match isbn_input.parse::<i64>() {
            Ok(isbn) => {
                commands::checkout_book(isbn.to_string(), borrower_name).await;
            }
            Err(_) => println!("Invalid input. Please enter a valid ISBN number, 'return', or 'exit'."),
        }
    }
}

async fn add_books() {
    loop {
        println!("Enter an ISBN (or type 'return' to return to the main menu:");
        let input = read_user_input().trim().to_lowercase();

        if input == "return" {
            break;
        } else if input == "exit" {
            println!("Exiting the program.");
            std::process::exit(0);
        }

        match input.parse::<i64>() {
            Ok(isbn) => {
                commands::add_book(isbn.to_string()).await;
            }
            Err(_) => println!("Invalid input. Please enter a valid ISBN number, 'return', or 'exit'."),
        }
    }
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

