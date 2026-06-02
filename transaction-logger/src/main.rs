use std::{
    collections::HashMap, fmt::Display, fs::{self, File, write}, io::{self, Write, stdin}
};

enum TransactionSide {
    Buy,
    Sell,
}

struct Transaction {
    name: String,
    amount: f64,
    price: f64,
    side: TransactionSide,
}

impl Display for TransactionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionSide::Buy => write!(f, "Buy"),
            TransactionSide::Sell => write!(f, "Sell"),
        }
    }
}

fn main() {
    let mut prices = HashMap::new();
    prices.insert("BTC".to_string(), 89000.0);
    prices.insert("BITCOIN".to_string(), 89000.0);
    prices.insert("ETH".to_string(), 2500.0);
    prices.insert("ETHEREUM".to_string(), 2500.0);
    prices.insert("SOL".to_string(), 98.0);
    prices.insert("SOLANA".to_string(), 98.0);

    println!(
        "Welcome to Transaction Logger
------------------
1. Buy
2. Sell
3. View history
4. Summary
5. Print dollar balance
6. Quit
------------------"
    );

    let mut transaction: Vec<Transaction> = Vec::new();
    let mut dollar_balance: f64 = 100000.0;

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim().parse::<u32>().unwrap_or(5);

        match input {
            1 => {
                println!("Enter token to buy");
                let mut token = String::new();
                stdin().read_line(&mut token).expect("Failed to read");
                let token = token.trim().to_uppercase();
                if let Some(price) = prices.get(&token) {
                    println!("Enter amount");
                    let mut amount = String::new();
                    stdin().read_line(&mut amount).expect("Failed to read");
                    let amount = amount.trim().parse::<f64>().unwrap_or(0.0);

                    if amount * price > dollar_balance {
                        println!("Insufficient funds.")
                    } else {
                        dollar_balance -= amount * price;
                        let transaction_details = Transaction {
                        name: token.clone(),
                        price: *price,
                        amount: amount,
                        side: TransactionSide::Buy,
                    };
                    println!("Bought {} {} at {}", amount, token, price);
                    transaction.push(transaction_details);
                    }
                } else {
                    println!("Token not found, enter a valid token from the list: BTC, ETH, SOL")
                }
            }
            2 => {
                println!("Enter token to sell");
                let mut token = String::new();
                stdin().read_line(&mut token).expect("Failed to read");
                let token = token.trim().to_uppercase();
                if let Some(price) = prices.get(&token) {
                    println!("Enter amount");
                    let mut amount = String::new();
                    stdin().read_line(&mut amount).expect("Failed to read");
                    let amount = amount.trim().parse::<f64>().unwrap_or(0.0);

                    let buy_total: f64 = transaction
                        .iter()
                        .filter(|t| t.name == token && matches!(t.side, TransactionSide::Buy))
                        .map(|t| t.amount)
                        .sum();

                    let sell_total: f64 = transaction
                        .iter()
                        .filter(|t| t.name == token && matches!(t.side, TransactionSide::Sell))
                        .map(|t| t.amount)
                        .sum();

                    let balance = buy_total - sell_total;

                    if amount > balance {
                        println!("Insufficient balance. You only have {} {}", balance, token);
                    } else {
                        let transaction_details = Transaction {
                            name: token.clone(),
                            price: *price,
                            amount: amount,
                            side: TransactionSide::Sell,
                        };
                        transaction.push(transaction_details);
                        dollar_balance += amount * price;
                    }
                } else {
                    println!("Token not found, enter a valid token from the list: BTC, ETH, SOL")
                }
            }
            3 => {
                println!("Here are your pasts transactions:");
                for (i, detail) in transaction.iter().enumerate() {
                    println!(
                        "[{}] {} {} {} at {} = {}",
                        i + 1,
                        detail.side,
                        detail.amount,
                        detail.name,
                        detail.price,
                        detail.amount * detail.price
                    )
                }
            }
            4 => {
                let total_spent: f64 = transaction
                    .iter()
                    .filter(|t| matches!(t.side, TransactionSide::Buy))
                    .map(|t| t.amount * t.price)
                    .sum();
                println!("Total spent: {}", total_spent);

                let total_earned: f64 = transaction
                    .iter()
                    .filter(|t| matches!(t.side, TransactionSide::Sell))
                    .map(|t| t.amount * t.price)
                    .sum();
                println!("Total earned: {}", total_earned);
            }
            5 => println!("{}", dollar_balance),
            6 => {
                println!("Thanks for trading with us");
                break;
            }
            _ => {
                println!(
                    "Invalid input, enter a valid input from 1-4 or press 5 to end this session"
                );
            }
        }
        println!(
            "Enter choice
------------------
1. Buy
2. Sell
3. View history
4. Summary
5. Print Dollar balance
6. Quit
------------------"
        );
    }
}

fn save_transactions(transactions: &Vec<Transaction>, dollar_balance: f64) -> Result<(), io::Error> {
    let mut contents = format!("{}\n", dollar_balance);
    for t in transactions {
        contents.push_str(&format!("{},{},{},{}\n", t.name, t.price, t.amount, t.side));
    }
    fs::write("transactions.txt", contents)
}


fn load_transactions(transactions: &mut Vec<Transaction>, dollar_balance: &mut f64) -> Result<(), io::Error> {
   let contents = match fs::read_to_string("transactions.txt") {
    Ok(c) => c,
    Err(e) => return Err(e),
   };

   for (i, line) in contents.split("\n").enumerate() {
    if i == 0 {
    *dollar_balance = line.parse::<f64>().unwrap_or(100000.0);
} else {
    let parts: Vec<&str> = line.split(',').collect();
    let transaction = Transaction {
      name: parts[0].to_string(),
    price: parts[1].parse::<f64>().unwrap_or(0.0),
    amount: parts[2].parse::<f64>().unwrap_or(0.0),
    side: match parts[3]{
        "Buy" => TransactionSide::Buy,
        _ => TransactionSide::Sell,
    }
    };
    transactions.push(transaction);
}
}
Ok(())
}