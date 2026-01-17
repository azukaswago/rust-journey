fn main() {
    const LIMIT: i32 = 1500;

    let mut balance = 5000;
    let expenses = [800, 2100, 500, 2000, 1000];
    let mut day = 1;
    let mut success = 0;
    let mut failure = 0;

    for expense in expenses {
        println!("Day {day}");
        day += 1;

        if expense <= LIMIT {
            if expense <= balance {
                balance -= expense;
                println!("Transaction of {expense} approved, new balance is {balance}");
                success += 1
            } else {
                failure += 1;
                println!(
                    "Transaction of {expense} failed due to insufficient funds, balance is {balance}."
                )
            }
        } else {
            println!(
                "{expense} is above daily limit, transaction couldn't be processed. Balance is {balance} and daily limit is {LIMIT}"
            );
            failure += 1
        };
    }
    println!("\nSummary:");
    println!("{success} successful tnxs. {failure} failed tnxs. Gross balance is {balance}")
}
