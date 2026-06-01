use std::io;
#[derive(Debug)]
struct Transaction {
    amount: f64,
    label: String,
}

fn print_all<F: Fn(&Transaction)>(transactions: &Vec<Transaction>, f: F) {
    for transaction in transactions {
        f(transaction);
    }
}

fn print_transaction(t: &Transaction) {
    println!("{}: ${}", t.label, t.amount)
}



fn main(){

let transaction_one = Transaction {
    amount: 50.0,
    label: String::from("Debit"),
};
let transaction_two = Transaction {
    amount: 60.0,
    label: String::from("Credit"),
};
    let mut transactions = vec![transaction_one, transaction_two];

    loop {
        let mut choice = String::new();
        println!("Enter 1 to filter by minimum amount, 2 to add print all transactions, 3 to add a transaction and 4 to quit:");
        io::stdin().read_line(&mut choice).expect("Failed to read input"); 
        match choice.trim() {
            "1" => {
                let mut amount = String::new();
                println!("Enter minimum amount to filter by:");
                io::stdin().read_line(&mut amount).expect("Failed to read input");
                let min_amount = amount.trim().parse::<f64>().unwrap_or(0.0);

               transactions.retain(|t| t.amount >= min_amount);
            },
            "2" => {
                print_all(&transactions, print_transaction);
                }

            "3" => {
                let mut new_amount = String::new();
                let mut new_label = String::new();

                println!("Enter amount");
                io::stdin().read_line(&mut new_amount).expect("Failed to read");
                println!("Enter label (Is this a credit or debit?)");
                io::stdin().read_line(&mut new_label).expect("Failed to read");

                let new_transaction = Transaction {
                    amount: new_amount.trim().parse::<f64>().unwrap_or(0.0),
                    label: new_label.trim().to_string(),
                };

                transactions.push(new_transaction);
            }

            
            "4" => {
                println!("Exiting...");
                break;
            },

            _ => {
                println!("Invalid choice, please try again.");
               
            }
        }
    }
}
