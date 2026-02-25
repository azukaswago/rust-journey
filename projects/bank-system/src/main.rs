//Bank system to practice enums and structs

#[derive(Debug)]
enum AccountType {
    Savings(f64),
    Checking,
    Business(String),
}

#[derive(Debug)]
struct BankAccount {
    owner_name: String,
    account_type: AccountType,
    current_balance: f64,
}

impl BankAccount {
    fn new(owner_name: String, account_type: AccountType, current_balance: f64) -> Self {
        BankAccount {
            owner_name,
            account_type,
            current_balance: 0.0,
        }
    }

    fn deposit(&mut self, transfer: f64) {
        self.current_balance = self.current_balance + transfer;
    }

    fn withdraw(&mut self, spending: f64) {
        if spending < self.current_balance {
            self.current_balance = self.current_balance - spending;
            println!(
                "Transaction approved, new balance is {}",
                self.current_balance
            )
        } else {
            println!(
                "Withdrawal failed due to insufficient funds, balance is {} ",
                self.current_balance
            )
        }
    }

    fn display_account_info(&self) {
        println!("=== Account Information ===");
        println!("Owner:{}", self.owner_name);

        match &self.account_type {
            AccountType::Savings(rate) => {
                println!("Type: Savings Account ({}% interest)", rate * 100.0)
            }
            AccountType::Checking => {
                println!("Type: Checking Account")
            }
            AccountType::Business(name) => {
                println!("Type: Business Account ({})", name)
            }
        }

        println!("Balance: ${:.2}", self.current_balance);
        println!("==============================\n");
    }
}

fn main() {
    let mut azuka_account = BankAccount::new(String::from("Azuka"), AccountType::Checking, 0.0);
    azuka_account.deposit(100_000_000.0);
    println!("Azuka current balance is {}", azuka_account.current_balance);
    azuka_account.withdraw(7_890_000.0);

    azuka_account.display_account_info();

    let mut santan_account = BankAccount::new(
        String::from("Santan"),
        AccountType::Business(String::from("Santan 2036 Sports Club")),
        0.0,
    );
    santan_account.deposit(1_000_000_000_654.98);
    println!(
        "Santan current balance is {}",
        santan_account.current_balance
    );
    santan_account.withdraw(7_890_348_656_000.0);

    santan_account.display_account_info();

    let mut obi_account = BankAccount::new(String::from("Obi"), AccountType::Savings(0.05), 0.0);
    obi_account.deposit(1_000_000_000_654.98);
    println!("Obi current balance is {}", obi_account.current_balance);
    obi_account.withdraw(90_348_656_000.0);

    obi_account.display_account_info();
}
