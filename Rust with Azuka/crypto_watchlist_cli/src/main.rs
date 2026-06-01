mod watchlist;
mod api;
use std::io:: stdin;

use crate::api::fetch_price;
use crate::watchlist::load_watchlist;
use crate::watchlist::save_watchlist;
fn main() {
    let mut watchlist = load_watchlist();
    loop {
         println!("1. Add token");
        println!("2. View watchlist");
        println!("3. Remove watchlist");
        println!("4. Quit");

        let mut choice = String::new();
        println!("Enter your choice");

        stdin().read_line(&mut choice)
        .expect("Failed to read your choice");
      match choice.trim() {
        "1" => {
            println!("Enter token name");
            let mut token = String::new();
            stdin().read_line(
                &mut token
            ).expect("Failed to read");
            watchlist.push(token.trim().to_string());
            save_watchlist(&watchlist);
        },
        "2" => {
            println!("Your watchlist:");
            for token in &watchlist {
                match fetch_price(&token.to_lowercase()) {
                    Some(s) => println!("{}: ${}",token, s),
                    None => println!("Token not found.")
                }
            }
        },
        "3" => {
            let mut token = String::new();
            println!("Enter token name to remove");
            stdin().read_line(
                &mut token
            ).expect("Failed to read");
            let token = token.trim();
            watchlist.retain(|w| w.trim() != token);
            save_watchlist(&watchlist);
        },
        "4" => {
            break;
        },
        _ => {
            println!("choice is unrecognized")
        },
    }
    }

  
}
