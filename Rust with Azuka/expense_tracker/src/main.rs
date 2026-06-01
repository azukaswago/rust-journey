mod expense;
mod storage;

use crate::{expense::Expense, storage::{load_expenses, save_expenses}};

use std::io;
    fn main() {
        let mut expenses = load_expenses();
    loop {
        println!("1. Add expense");
        println!("2. View expenses");
        println!("3. Total");
        println!("4. Quit");

        let mut choice = String::new();
        println!("Enter your choice");
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
    "1" => {
        let mut name = String::new();
        let mut amount = String::new();
        let mut category =  String::new();

        println!("Enter expense name");
        io::stdin().read_line(&mut name).expect("Failed to read name");

        println!("Enter expense amount");
        io::stdin().read_line(&mut amount).expect("Failed to read name");
        let amount_val = match amount.trim().parse::<f64>() {
            Ok(val) => val,
            Err(_) => {println!("invalid amount"); continue;}
        };

        println!("Enter expense category");
        io::stdin().read_line(&mut category).expect("Failed to read name");

        let expense = Expense::new(name.trim().to_string(), amount_val, category.trim().to_string());
        expenses.push(expense);
        save_expenses(expenses.clone());

    },
    "2" => {
        for expense in &expenses {
            expense.display();
        }
    },
    "3" => {
        let mut total = 0.0;
        for expense in &expenses {
            total = expense.amount + total
        }
        println!("{}", total);
    },
    "4" => break,
    _ => println!("Invalid option"),
}
    }

    
}
