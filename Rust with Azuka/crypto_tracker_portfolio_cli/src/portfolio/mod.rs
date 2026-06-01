#[derive(Debug, Clone)]
pub struct Coin{
    pub name: String,
    pub amount: f64,
    pub price_in_USD: f64,
}

#[derive(Debug)]
pub struct Portfolio {
   pub coins:  Vec<Coin>,
}


impl Portfolio {
   pub fn new() -> Self {
    Portfolio { 
        coins: Vec::new() 
    }
   }

   pub fn add_coin(&mut self, coin: Coin) {
    self.coins.push(coin)
   }

   pub fn remove_coin(&mut self, name: &str) {
   let mut new_coins = Vec::new();
for coin in &self.coins {
    if coin.name != name {
        new_coins.push(coin.clone());
    }
}
self.coins = new_coins;
}

   pub fn total_value(&self) -> f64 {
    let mut total = 0.0;
    for coin in &self.coins {
        total += coin.amount * coin.price_in_USD
    };
    total
   }

   pub fn display(&self) {
    println!("")
   }
}















// Got it, less handholding.
// Here's the full scope for portfolio/mod.rs — implement it yourself:

// Coin struct with name, amount, price_usd
// Portfolio struct holding a Vec<Coin>
// Methods on Portfolio: new, add_coin, remove_coin, total_value, display

