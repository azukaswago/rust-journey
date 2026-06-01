pub fn fetch_price(coin: &str) -> Option<f64>  {
   let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={coin}&vs_currencies=usd");
  let client = reqwest::blocking::Client::new();
let response = client.get(&url)
    .header("User-Agent", "crypto-watchlist-cli/1.0")
    .send().ok()?;
   let json: serde_json::Value = response.json().ok()?;
   let price = json[coin]["usd"].as_f64()?;
   return Some(price)
}