// Portfolio Tracker CLI

// Store a list of assets and amounts (e.g. BTC: 0.5, ETH: 2.0)
// Print the portfolio
// Add and remove assets


use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut portfolio = HashMap::new();
    portfolio.insert("BTC".to_string(), 0.5);
    portfolio.insert("ETH".to_string(), 1.3);
    portfolio.insert("SOL".to_string(), 23.9);

    println!("Enter your name:");
    let mut user = String::new();
    stdin().read_line(&mut user).expect("failed to read");
    user = user.trim().to_string();
    println!(
        "Welcome {}, Press:\n1 to view compact portfolio\n2 to print portfolio\n3 to remove token\n4 to add token\n5 to quit",
        user
    );

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed to read");
        let input = input.trim().parse::<u32>().unwrap_or(5);

        match input {
            1 => print_compact(&portfolio, &user),
            2 => print_portfolio(&user, &portfolio),
            3 => {
                println!("Enter token to remove");
                let mut choice = String::new();
                stdin().read_line(&mut choice).expect("failed to read");
                let choice = choice.trim().to_string();

                match remove(&mut portfolio, choice) {
                    Some(amount) => println!("Removed {} tokens", amount),
                    None => println!("Token not found in portfolio"),
                }
            }
            4 => {
                println!("Enter token to add");
                let mut choice = String::new();
                stdin().read_line(&mut choice).expect("failed to read");
                let choice = choice.trim().to_string();

                println!("Enter token amount");
                let mut amount = String::new();
                stdin().read_line(&mut amount).expect("failed to read");
                let amount = amount.trim().parse::<f64>().unwrap_or(0.0);

                match add(&mut portfolio, &choice, amount) {
                    Some(old_amount) => println!("Updated {} from {} to {}", choice, old_amount, amount),
                    None => println!("Added {} with amount {}", choice, amount),
                }
            }
            5 | _ => {
                break;
            }
        }
    }
}

fn print_portfolio(user: &String, portfolio: &HashMap<String, f64>) {
    for (token, amount) in portfolio {
        println!("{} has {} {} tokens in his portfolio", user, token, amount)
    }
}

fn print_compact(portfolio: &HashMap<String, f64>, user: &String) {
    println!("{}'s port: {:?}", user, portfolio)
}

fn remove(portfolio: &mut HashMap<String, f64>, choice: String) -> Option<f64> {
    portfolio.remove(&choice)
}

fn add(portfolio: &mut HashMap<String, f64>, key: &str, value: f64) -> Option<f64> {
    portfolio.insert(key.to_string(), value)
}
