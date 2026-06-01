// mod traits;
// mod getter;
// use getter::Trader;
// use getter::HasProfile;
// use traits::TradableAsset;
// use traits::CryptoAsset;
// use traits::ForexPair;

// fn main() {
//     let crypto_asset = CryptoAsset {
//         asset: String::from("BTC"),
//         price: 94_500.94,
//         max_leverage: 100,
//     };

//     let forex_pair = ForexPair {
//         pair: String::from("NGN/USD"),
//         price: 0.00063,
//         max_leverage: 100,
//     };

//     let port: Vec<Box<dyn TradableAsset>> = vec![
//         Box::new(crypto_asset), Box::new(forex_pair)
//     ];

//         for p in &port {
//             println!("{}", p.risk_profile())
//         } ;

//         let trader_profile = Trader {
//             username: String::from("Santan"),
//             age: 19,
//         };

//         print_trader_profile(&trader_profile);
// }
//   fn print_trader_profile (trader: &impl HasProfile){
//         println!("{} is {}",trader.username(), trader.age() )
//     }

// pub trait TradingAccount {
//     fn owner(&self) -> &str;
//     fn balance(&self) -> f64;
//     fn set_balance (&mut self, amount: f64);
//     fn deposit (&mut self, amount: f64);
//     fn withdraw (&mut self, amount: f64);
//     fn statement(&self) -> String {
//         format!("{} is closing with a balance of {}", self.owner(), self.balance())
//     }
// }

// mod getter;
// use getter::TradingAccount;
// use getter::Account;

// fn main(){
//     let mut account = Account {
//          holder: String::from("Obi Chidera Azuka"),
//          ledger_balance: 7_900_700.90,
//     };
//     println!("{}", account.statement());
//     account.set_balance(9_000_000_000.24);
//     println!("-----------------");
//     println!("{}", account.notis());
 
//     account.deposit(400_000.87);
//       println!("{}", account.notis());
//      account.withdraw(800_000.90);
//      println!("{}", account.notis());
//      account.withdraw(10_800_000.90);  
//       println!("{}", account.statement());
     

// }

mod supertraits;
use supertraits::CryptoAsset;
use supertraits:: Tradeable;
use supertraits::Asset;
fn main(){
  let crypto_asset =  CryptoAsset {
        ticker: String::from("Popzilla on SOL"),
        price: 0.00899,
    };

    crypto_asset.summary();
}