use std::fmt::Display;


trait Asset {
    fn ticker(&self) -> &str;
}

trait Tradeable: Asset {
    fn price(&self) -> f64;
    fn summary(&self) {
        println!("Ticker: {}, price: {}", self.ticker(), self.price())
    }
}

trait Reportable {
    type Report;
    fn generate_report(&self) -> Self::Report ;
}


#[derive(Clone)]
struct CryptoToken {
    ticker: String,
    price: f64,
    holders: u64,
}


#[derive(Clone)]
struct StockAsset {
    ticker: String,
    price: f64,
    holders: u64,
}

impl Asset for CryptoToken {
    fn ticker(&self) -> &str {
        &self.ticker
    }
}

impl Tradeable for CryptoToken  {
    fn price(&self) -> f64 {
        self.price
    }
}

impl Asset for StockAsset {
    fn ticker(&self) -> &str {
        &self.ticker
    }
}

impl Tradeable for StockAsset  {
    fn price(&self) -> f64 {
        self.price
    } 
}

impl Reportable for CryptoToken  {
    type Report = String;
    fn generate_report(&self) -> Self::Report {
            format!("{} holders of {} at a price of ${}", self.holders, self.ticker, self.price)
    }
}

impl Reportable for StockAsset  {
    type Report = String;
    fn generate_report(&self) -> Self::Report {
            format!("At a price of ${}, {} has {} holders", self.price, self.ticker, self.holders)
    }
}

impl Display for CryptoToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, ${}, {}", self.ticker, self.price, self.holders)
    }   
}

impl Display for StockAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, ${}, {}", self.ticker, self.price, self.holders)
    }   
}

impl PartialEq for CryptoToken  {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker
    }
}

impl PartialEq for StockAsset  {
    fn eq(&self, other: &Self) -> bool {
        self.ticker == other.ticker
    }
}

fn trade<T: Tradeable> (item: T) {
    item.summary();
}

fn main () {
    let crypto_token = CryptoToken {
        ticker: String::from("Popcat"),
        price: 0.09,
        holders: 2_098_987_876_900,
    };

      let crypto_token2 = CryptoToken {
        ticker: String::from("Popcat"),
        price: 0.19,
        holders: 987_876_900,
    };

    let stock_asset =StockAsset { ticker: String::from("SHELL/USD"), price: 3.0, holders:100_000 };

    trade(crypto_token.clone());
    trade(stock_asset.clone());

    println!("{}",  crypto_token.generate_report());
    println!("{}",  stock_asset.generate_report());
    println!("{}", crypto_token.eq(&crypto_token2));
    
    println!("{}", crypto_token);
    println!("{}", stock_asset);
    
}