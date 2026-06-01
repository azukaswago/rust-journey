// pub trait HasProfile {
//     fn username(&self) -> &str;
//     fn age(&self) -> u32;
// }

// pub struct Trader {
//    pub username: String,
//     pub age: u32,
// }

// impl HasProfile for Trader  {
//     fn username(&self) -> &str {
//         &self.username
//     }

//     fn age(&self) -> u32 {
//         self.age
//     }
// }

// Mixed exercise:

// Define a trait `TradingAccount` with:
// - Getters: `owner(&self) -> &str`, `balance(&self) -> f64`
// - Setters: `set_balance(&mut self, amount: f64)`, `deposit(&mut self, amount: f64)` — deposit should add to the existing balance, not replace it
// - A default method `statement(&self) -> String` that returns `"[owner] — balance: $[balance]"`

// Implement on a struct `Account` with private fields. In main, create an account, print statement, deposit, print statement again.

pub trait TradingAccount {
    fn owner(&self) -> &str;
    fn balance(&self) -> f64;
    fn set_balance (&mut self, amount: f64);
    fn deposit (&mut self, amount: f64);
    fn withdraw (&mut self, amount: f64);
    fn statement(&self) -> String {
        format!("{} is closing with a balance of ${:.2}", self.owner(), self.balance())
    }
    fn notis(&self) -> String {
        format!("Transaction made, new balance is ${}", self.balance())
    }
}

pub struct Account {
   pub holder: String,
   pub ledger_balance: f64,
}

impl TradingAccount for Account  {
    fn owner(&self) -> &str {
        &self.holder
    }

    fn balance(&self) -> f64 {
        self.ledger_balance
    }

    fn set_balance (&mut self, amount: f64) {
        self.ledger_balance = amount
    }

    fn deposit (&mut self, amount: f64) {
        self.ledger_balance += amount
    }

    fn withdraw (&mut self, amount: f64) {
        if amount <= self.ledger_balance {
            self.ledger_balance -= amount
        } else {
           println!("Transaction failed, insufficient funds")
        }
    }
}


