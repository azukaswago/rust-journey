use std::{fs, io};
fn main(){

let mut ticker = String::new();
println!("Enter ticker:");
io::stdin().read_line(&mut ticker).expect("Failed to read");

let mut price = String::new();
println!("Enter price:");
io::stdin().read_line(&mut price).expect("Failed to read");
let price:f64 = price.trim().parse().expect("Please enter a valid number");
println!("{} is trading at {}", ticker.trim(), price);

let user1_file = format!("Ticker: {}, price: {}",ticker.trim(), price);
fs::write("user1.txt", user1_file).expect("Cant't write file");
let contents = fs::read_to_string("user1.txt").expect("Failed to read");
println!("{}", contents);


}
