// trait Summarize {
//     fn summarize(&self) -> String {
//         String::from("No summary.")
//     }
// }

// #[derive(Debug, Clone)]
// struct Coin {
//     name: String,
//     price:f64,
// }

// #[derive(Debug)]
// struct Whale {
//     wallet_address: String,
//     coin: Coin,
//     amount_holding: u64,
// }

// impl Summarize for Coin {
//     fn summarize(&self) -> String {
//         format!("{} is currently being priced at ${}", self.name, self.price)
//     }
// }

// impl Summarize for  Whale {
//     fn summarize(&self) -> String {
//         format!("The biggest {} whale has wallet address {} and holds {} tokens.",
//         self.coin.name, self.wallet_address, self.amount_holding)
//     }
// }

// fn main() {

//   let boris =  Coin {
//         name: String::from("Boris on ETH"),
//         price: 0.00000876,
//     };

//     let whale = Whale {
//         wallet_address: String::from("Ox546bed645adcc64"),
//         coin: boris.clone(),
//         amount_holding: 89999999999999877771.90 as u64,
//     };

//    println!("{}", boris.summarize());
//    println!("{}", whale.summarize());
// }

// trait Wallet {
//    fn balance(&self) -> f64;

//    fn summary(&self) {
//     let balance = self.balance();
//     println!("Your balance is {}", balance)
//    }
// }

// #[derive(Debug)]
// struct CryptoWallet {
//     amount: f64,
// }

// impl Wallet for CryptoWallet {
//     fn balance(&self) -> f64 {
//         self.amount
//     }
// }

// fn main(){
//   let my_wallet =  CryptoWallet{
//         amount: 56.98,
//     };

//     my_wallet.summary();
// }

// trait Describe {
//      fn name(&self) -> String;

//     fn introduce(&self)-> String{
//         let name = self.name();
//         format!("I am {}", name)
//     }
// }

// #[derive(Debug, Clone)]
// struct Token {
//     symbol: String
// }

// impl Describe for Token {
//     fn name(&self) -> String {
//         self.symbol.clone()
//     }
// }

// fn main(){
// let token = Token {
//         symbol: String::from("BTC"),
//     };

//         println!("{}",token.introduce())
//   ;
// }

// trait Greet {
//     fn hello(&self)-> String;
// }

// struct User {
//     username: String,
// }

// impl Greet for User  {
//     fn hello(&self)-> String {
//         self.username.clone()
//     }
// }

// fn greet_user(can_use: &impl Greet) {
//     println!("{}", can_use.hello())
// }

// fn main(){
// let user = User {
//         username: String::from("Mrs Swago")
//     };

//     greet_user(&user);
// }

// trait Summarize {
//     fn content(&self) -> String;

//     fn preview(&self) -> String {
//         let content = self.content().clone();
//         format!("Preview: {}", content)
//     }
// }

// struct Post {
//     body: String,
// }

// impl Summarize for Post  {
//     fn content(&self) -> String {
//         self.body.clone()
//     }
// }

// fn print_preview(ty: &impl Summarize) {
//   println!("{}", ty.preview())
// }

// fn main(){
//     let news = Post {
//         body: String::from("Ethereum aint shit"),
//     };

//     print_preview(&news);
// }

// trait Rank {
//     fn position(&self) -> u32;
// }

// struct Player {
//     rank: u32,
// }

// impl Rank for Player {
//     fn position(&self) -> u32 {
//         self.rank
//     }
// }

// fn compare_ranks<T: Rank> (a: &T, b: &T) {
//     println!("Player1: {}, Player2: {}", a.position(), b.position())
// }

// fn main(){
//     let player1 = Player {
//         rank: 3
//     };

//     let player2 = Player {
//         rank: 4
//     };

//     compare_ranks(&player1, &player2);
// }

// trait Describe {
//   fn label(&self) -> String;
//   fn display(&self) -> String {
//    let label = self.label();
//     format!("Item: {}", label)
//   }
// }

// #[derive(Debug)]

// struct Coin {
//     name: String,
// }

// #[derive(Debug)]

// struct Token {
//     symbol: String,
// }

// impl Describe for Coin  {
//     fn label(&self) -> String {
//         self.name.clone()
//     }
// }

// impl Describe for Token  {
//     fn label(&self) -> String {
//         self.symbol.clone()
//     }
// }

// fn cray<T: Describe>(name: &T, symbol: &T) {
//     println!("{}, {}", name.display(), symbol.display())
// }

// fn main(){
//    let coin1 = Coin {
//         name: String::from("ETH")
//     };

//     let coin2 = Coin {
//         name: String::from("SOL")
//     };

//     let token = Token {
//         symbol: String::from("ZUKA")
//     };

//     cray(&coin1, &coin2);
// }

// use std::fmt::Debug;

// trait Greet {
//     fn hello(&self) -> String;
// }

// #[derive(Debug)]
// struct User {
//     username: String,
// }

// impl Greet for User {
//     fn hello(&self) -> String {
//         format!("Hello {}", self.username)
//     }
// }

// fn hi <T: Greet + Debug>(a: &T) {
//     println!("{}. {:?}", a.hello(), a)
// }

// fn main() {
//     let user = User{
//         username: String::from("Don Gorgon")
//     };
//     hi(&user);
// }

// use std::fmt::Debug;
// trait Describe {
//     fn label(&self) -> String;
//     fn display(&self) -> String {
//         format!("Item: {}", self.label())
//     }
// }

// #[derive(Debug)]
// struct Coin{
//     name: String,
// }

// impl Describe for Coin  {
//     fn label(&self) -> String {
//         self.name.clone()
//     }
// }

// fn fun<T: Describe + Debug>(a: &T) {
//     println!("{}. {:?}",a.display(), a )
// }

// fn main() {
//     let sol = Coin {
//         name: String::from("Solana")
//     };

//     fun(&sol);
// }

// use std::fmt::Debug;

// trait Notify {
//     fn message(&self) -> String;
//     fn urgent(&self) -> String {
//         format!("TIME SENSITIVE: {}", self.message())
//     }
// }

// #[derive(Debug)]
// struct Alert {
//     text: String,
// }

// impl Notify for Alert {
//     fn message(&self) -> String {
//         self.text.clone()
//     }
// }

// fn notis<T>(bell: &T)
// where
//     T: Notify + Debug,
// {
//     let message = bell.urgent();
//     println!("{} {:?}", message, bell)
// }

// fn main() {
//     let noti = Alert {
//         text: String::from("You have one new message from Aduuni mi"),
//     };
//     notis(&noti);
// }

// trait Generate {
//     fn value(&self) -> String ;
//     fn labeled(&self) -> String {
//         format!("Generated: {}", self.value())
//     }
// }

// struct Token {
//      name: String,
// }

// impl Generate for Token  {
//     fn value(&self) -> String {
//         self.name.clone()
//     }
// }

// fn make_token() -> impl Generate {
//     Token{
//         name: String::from("Ethereum")
//     }
// }

// fn main(){
//     let tok = make_token();
//     println!("{}", tok.labeled());
// }

// use std::fmt::Display;

// struct Container <T> {
//     value: T
// }

// impl<T: Display> Container<T>{
//     fn print_value(&self){
//         println!("{}", self.value)
//     }
// }

// fn main(){
//     let container = Container {
//         value: String::from("Value Added Tax")
//     };
//     container.print_value();
// }

// use std::fmt::Display;

// struct Holder<T> {
//     data: T,
// }

// trait Summarize {
//     fn summary(&self) -> String;
// }

// impl<T: Display> Summarize for Holder<T>  {
//     fn summary(&self) -> String {
//         format!("Data: {}", self.data)
//     }
// }

// fn main(){
//     let holder = Holder {
//         data: String::from("FUP limit reached")
//     };
//    println!("{}", holder.summary())
// }

// trait Sound {
//     fn make_sound(&self) -> String;
// }

// struct Dog {
//     name: String,
// }

// struct Cat {
//     name: String,
// }

// impl Sound for Dog  {
//     fn make_sound(&self) -> String {
//         self.name.clone()
//     }
// }

// impl Sound for Cat  {
//     fn make_sound(&self) -> String {
//         self.name.clone()
//     }
// }

// fn main (){
//     let dog = Dog {
//         name: String::from("bark"),
//     };

//     let cat = Cat {
//         name: String::from("mew"),
//     };

//     let sounds: Vec<Box<dyn Sound>> = vec![Box::new(dog), Box::new(cat)];

//     for sound in &sounds {
//         println!("{}", sound.make_sound())
//     }
//     // let sounds = vec![dog.make_sound(), cat.make_sound()];

// }

// Implement it on three structs — Coin with symbol: String, price: f64, Stock with ticker: String, price: f64, and RealEstate with address: String, worth: f64.
// Store all three in a Vec<Box<dyn Asset>>, loop through and print report() for each

// trait Asset {
//     fn value(&self) -> f64;
//     fn name(&self) -> String;
//     fn report(&self) -> String {
//         format!("{} is worth ${}", self.name(), self.value())
//     }
// }

// struct Coin {
//     symbol: String,
//     price: f64,
// }

// struct Stock {
//     ticker: String,
//     price: f64,
// }

// struct RealEstate {
//     address: String,
//     worth: f64,
// }

// impl Asset for Coin {
//     fn name(&self) -> String {
//         self.symbol.clone()
//     }

//     fn value(&self) -> f64 {
//         self.price
//     }
// }

// impl Asset for Stock {
//     fn name(&self) -> String {
//         self.ticker.clone()
//     }

//     fn value(&self) -> f64 {
//         self.price
//     }
// }

// impl Asset for RealEstate {
//     fn name(&self) -> String {
//         self.address.clone()
//     }

//     fn value(&self) -> f64 {
//         self.worth
//     }
// }

// fn main() {
//     let investments: Vec<Box<dyn Asset>> = vec![
//         Box::new(Coin {
//             symbol: String::from("Zuks"),
//             price: 450.98,
//         }),
//         Box::new(Stock {
//             ticker: String::from("Zuks Fitness Lab"),
//             price: 9.89,
//         }),
//         Box::new(RealEstate {
//             address: String::from("7/9 Dida Crescent, Kano Municipal, Kano State, Nigeria"),
//             worth: 8_000_085.78
//         }),
//     ];

//     for investment in &investments {
//         println!("{}", investment.report())
//     }

// }

// use std::fmt;


// struct TradeOrder {
//     pair: String,
//     amount: f64,
//     side: String,
// }

// enum OrderSide {
//     BUY,
//     SELL,
// }

// impl fmt::Display for OrderSide {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self{
//             OrderSide::BUY => write!(f,"BUY"),
//             OrderSide::SELL => write!(f, "SELL")
//         }
//     }
// }

// impl fmt::Display for TradeOrder  {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {} OF {}", self.side, self.amount, self.pair)
//     }
// }

// fn main(){
//    let order =  TradeOrder{
//         pair: String::from("BTC/USD"),
//         amount: 0.5,
//         side: String::from("BUY"),
//     };

//     let first_transaction = OrderSide::BUY;
//     println!("{}",  first_transaction);

//     println!("{}", order);

// }

// use std::fmt;

// struct Position {
//     asset: String,
//     quantity: f64,
//     entry_price: f64,
// }

// impl fmt::Debug for Position {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Position(asset = {}, qty = {}, entry = {})", self.asset, self.quantity, self.entry_price)
//     }
// }

// fn main(){
//   let position = Position {
//     asset: String::from("Shell_Co"),
//     quantity: 623.90,
//     entry_price: 8900000.90,
//   };

//   println!("{:?}", position);
// }

// Create a struct Order with fields: id: u32, pair: String, amount: f64, internal_flag: bool
// Implement Debug manually using f.debug_struct() — include id, pair, and amount, but exclude internal_flag
// Print it with {:?} in main

// use std::fmt;

// struct Order {
//     id: u32,
//     pair: String,
//     amount: f64,
//     internal_flag: bool,
// }

// impl fmt::Debug for Order  {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Order") .field("id", &self.id).field("pair", &self.pair).field("amount", &self.amount) .finish()
//     }
// }
// fn main(){
//     let order = Order {
//         id: 6,
//         pair: String::from("meh"),
//         amount: 7.9,
//         internal_flag: true,
//     };

//     println!("{:?}",order);
// }

// struct ApiConnection {
//     endpoint: String,
// }

// impl Drop for ApiConnection  {
//     fn drop(&mut self) {
//         println!("Disconnecting from {}", self.endpoint)
//     }
// }

// fn main(){
//    let api_connection =  ApiConnection {
//         endpoint: String::from("endpoint")
//     };
//     println!("Connecting to server")
// }

// struct Wallet {
//     address: String,
//     balance: f64,
// }

// impl Clone for Wallet {
//     fn clone(&self) -> Self {
//         Wallet {
//             address: self.address,
//             balance: self.balance,
//         }
//     }
// }


// // 
// fn main(){
//     let wallet = Wallet {
//         address: String::from("0x56hf6838"),
//         balance: 54.8,
//     };
// }

// use std::fmt;

// #[derive(Clone)]
// struct Trade {
//     id: u32,
//     pair: String,
//     side: String,
//     amount: f64,
// }

// impl PartialEq for Trade  {
//     fn eq(&self, other: &Self) -> bool {
//         self.pair == other.pair && self.side == other.side
//     }
// }

// impl fmt::Display for Trade  {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"id {} made a transaction worth {} from the {} side on {}", self.id, self.amount, self.side, self.pair)
//     }
// }


// fn main() {
//     let trade = Trade {
//         id: 3,
//         pair: String::from("BTC/USD"),
//         side: String::from("wtf is a side"),
//         amount: 5000.0,
//     };

//     let trade2 = trade.clone();

//     let trade3 = Trade {
//         id: 3,
//         pair: String::from("GOLD/USD"),
//         side: String::from("SELL"),
//         amount: 5000.0,
//     };

//     println!("{}", trade == trade2); 
//     println!("{}", trade == trade3); 
//     println!("{}", trade != trade3); 

//     println!("{}", trade);

// }

// struct Asset {
//  ticker: String,
//  price: f64,   
// }

// impl PartialEq<String> for Asset  {
//     fn eq(&self, other: &String) -> bool {
//         &self.ticker == other
//     }
// }

// fn main(){
//     let ticker1 = String::from("Popcat");
//     let ticker2 = String::from("Lolcat");

//     let alpha_ticker = Asset {
//         ticker: String::from("Popcat"),
//         price: 67.8,
//     };

//     println!("{}", alpha_ticker.eq(&ticker1));
//      println!("{}", alpha_ticker.eq(&ticker2));
// }

// enum OrderType{
//     Market,
//     Limit(f64),
// }

// impl PartialEq for OrderType {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (OrderType::Market,OrderType::Market) => true,
//             (OrderType::Limit(x), OrderType::Limit(y)) => x == y,
//             _=> false,
//         }
//     }
// }

// fn main(){
//     println!("{}", OrderType::Market.eq(&OrderType::Market));
//      println!("{}", OrderType::Limit(1.0).eq(&OrderType::Limit(1.0)));
//       println!("{}", OrderType::Limit(1.0).eq(&OrderType::Market));
//     println!("{:?}", OrderType::Limit(1.0).eq(&OrderType::Limit(0.9)));
// }


// struct Currency {
//     code: String,
// }

// impl PartialEq for Currency  {
//     fn eq(&self, other: &Self) -> bool {
//         self.code == other.code
//     }
// }

// impl Eq for Currency  {
    
// }

// fn main(){
//     let germany = Currency {
//         code: String::from("EURO")
//     };

//      let nigeria = Currency {
//         code: String::from("NGN")
//     };

//     let latvia = Currency {
//         code: String::from("EURO")
//     };

//     println!("{}", germany.eq(&latvia));
//      println!("{}", germany.eq(&nigeria));
// }


trait Converter {
    type Output;
    fn convert(&self) -> Self::Output;
}

struct CryptoAmount {
    lamports: u64
}

impl Converter for CryptoAmount {
     type Output = f64;
    fn convert(&self) -> Self::Output {
        self.lamports as f64 /1_000_000_000.00
    }
}

fn show_value<T: Converter<Output = f64>>(item: T) {
    println!("{} worth of SOL", item.convert())
}

fn main(){
    let amount = CryptoAmount { lamports:7_000_000_000_000 };
    show_value(amount);
}