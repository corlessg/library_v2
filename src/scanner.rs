use std::io;
use crate::commands;

pub async fn scan_books() {
    
    // choose an operating mode... query, check-in, check-out

    println!("Query, add, checkin or checkout books? (or type 'exit' to quit):");

    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Failed to read line");

    let action_list = ["query","checkin","checkout","exit"];

    // Trim whitespace and convert to lowercase for case-insensitive comparison
    action = action.trim().to_lowercase();

    if action_list.contains(&action.as_str()) {
        if action == "exit" {
            println!("Exiting the program.");
        }
        else if action == "checkin" {
            loop {
                println!("Enter an ISBN (or type 'exit' to quit):");
        
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
        
                // Trim whitespace and convert to lowercase for case-insensitive comparison
                let input = input.trim().to_lowercase();
        
                if input == "exit" {
                    println!("Exiting the program.");
                    break;
                }
        
                // Parse the input as a number
                match input.parse::<i64>() {
                    Ok(number) => {
                        commands::checkin_book(number.to_string()).await;
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid ISBN number or 'exit'.");
                    }
                }
            }
        }
        else if action == "checkout" {
            loop {

                println!("Enter borrower's name");
                let mut borrower_name = String::new();
                io::stdin().read_line(&mut borrower_name).expect("Failed to read line");

                println!("Enter an ISBN (or type 'exit' to quit):");
        
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
        
                // Trim whitespace and convert to lowercase for case-insensitive comparison
                let input = input.trim().to_lowercase();
        
                if input == "exit" {
                    println!("Exiting the program.");
                    break;
                }
        
                // Parse the input as a number
                match input.parse::<i64>() {
                    Ok(number) => {
                        commands::checkout_book(number.to_string(),borrower_name).await;
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid ISBN number or 'exit'.");
                    }
                }
            }
        }
        else if action == "add" {
            loop {

                println!("Enter an ISBN (or type 'exit' to quit):");
        
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
        
                // Trim whitespace and convert to lowercase for case-insensitive comparison
                let input = input.trim().to_lowercase();
        
                if input == "exit" {
                    println!("Exiting the program.");
                    break;
                }
        
                // Parse the input as a number
                match input.parse::<i64>() {
                    Ok(number) => {
                        commands::add_book(number.to_string()).await;
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid ISBN number or 'exit'.");
                    }
                }
            }
        }
        
    }
    else {

        println!("You did not choose an acceptable action."); 
    }
    
    
    
}