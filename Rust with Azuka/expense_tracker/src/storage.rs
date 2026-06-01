use std::fs;

use crate::expense::Expense;

pub fn save_expenses(lists: Vec<Expense>){
    let mut contents = String::new();
    for list in &lists {
        let content =format!("{}, {}, {}\n", list.name, list.amount, list.category);
        contents.push_str(&content);
    }

    fs::write("expenses.csv", contents).expect("Failed to write");
}
pub fn load_expenses() -> Vec<Expense> {
   let contents = match fs::read_to_string("expenses.csv") {
    Ok(c) => c,
    Err(e) => return Vec::new()
   };
   let mut expenses: Vec<Expense> = Vec::new();

   for line in contents.lines() {
       let parts: Vec<&str> = line.split(',').collect();
       let amount: f64 = parts[1].trim().parse().expect("Invalid amount");
       let expense = Expense::new(parts[0].to_string(), amount, parts[2].to_string());
       expenses.push(expense);
   }
          expenses
}