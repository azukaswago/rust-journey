// Define a trait Asset with one required method: fn ticker(&self) -> &str
// Define a supertrait Tradeable: Asset with one required method: fn price(&self) -> f64, 
// and one default method fn summary(&self) that prints: "[TICKER] is trading at $[PRICE]" — using self.ticker() and self.price() inside it
// Create a struct CryptoAsset with fields ticker: String and price: f64
// Implement both traits for CryptoAsset
// Call .summary() on a CryptoAsset instance

pub trait Asset {
    fn ticker(&self) -> &str ;
}

pub trait Tradeable: Asset {
    fn price(&self) -> f64;
    fn summary(&self) {
        println!("{} is trading at ${}", self.ticker(), self.price())
    }
}

pub struct CryptoAsset {
   pub ticker: String,
   pub price: f64,
}

impl Asset for CryptoAsset {
    fn ticker(&self) ->&str {
        &self.ticker
    }
}

impl Tradeable for CryptoAsset {
    fn price(&self) -> f64 {
        self.price
    }
}

