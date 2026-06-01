// use core::fmt;
// use std::{collections::HashMap, fmt::Display, i32, num::ParseIntError};

// fn main() {
//     let mut league:HashMap<String, i32> = HashMap::new();
//     league.insert(String::from("RealMadrid"), 90);
//     league.insert(String::from("Barcelona"), 83);
//     league.insert(String::from("Sevilla"), 76);
//     league.insert(String::from("Osasuna"), 64);

//     let info = league.get("RealMadrid");
//     match info {
//         Some(s) => println!("Real Madrid qualified for the UCL league phase with {} points", s),
//         None => println!("Real Madrid missed out on UCL league phase qualification finishing out of the top 4"),
//     }

//     let a = 4.5;
//     let b = 0.2 ;
//     let c = 0.0;

//     match divide(a, b){
//         Ok(s) => println!("Value after division is {}", s),
//         Err(_) => println!("Cannot divide by zero"),
//     };

//       match divide(a, c){
//         Ok(s) => println!("Value after division is {}", s),
//         Err(e) => println!("{}", e),
//     };

//     let parseone = "42";
//     let parsetwo = "abc";

//     match parse_and_double(parseone) {
//         Ok(n) => println!("Parsed and doubled to give {}", n),
//         Err(q) => println!("{}", q),
//     }

//       match parse_and_double(parsetwo) {
//         Ok(n) => println!("Parsed and doubled to give {}", n),
//         Err(q) => println!("{}", q),
//     }

//     let neg = -2.2;
//     let pos = 9.0;
//     let zero = 0.0;
//     let postwo = 6.0;

//     match calc(neg, pos) {
//         Ok(ans) => println!("{}", ans),
//         Err(e) => println!("{}", e),
//     }

//      match calc(pos, zero) {
//         Ok(ans) => println!("{}", ans),
//         Err(e) => println!("{}", e),
//     }

//      match calc(pos, postwo) {
//         Ok(ans) => println!("{}", ans),
//         Err(e) => println!("{}", e),
//     }
// } 

// fn divide(a:f64, b:f64) -> Result<f64, String> {
//     if b == 0.0 {
//         Err(String::from("Cannot divide by 0"))
//     } else {
//         Ok(a/b)
//     }
// }

// fn parse_and_double(wond: &str) -> Result<i32, ParseIntError> {
//  let n = wond.parse::<i32>()?;
// Ok(n*2)
// }


// #[derive(Debug)]
// enum CalcError {
//     DivisionByZero,
//     NegativeInput,
// }

// impl fmt::Display for  CalcError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::DivisionByZero => write!(f, "cannot divide by 0"),
//             Self::NegativeInput => write!(f, "negative input"),
//         }
//     }
// } 

// fn calc(a:f64, b:f64) -> Result<f64, CalcError> {
//     if b < 0.0 || a < 0.0 {
//         Err(CalcError::NegativeInput)
//     } else if b == 0.0 {
//         Err(CalcError::DivisionByZero)
//     }
//     else {
//         Ok(a/b)
//     }
// }


use::std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let input = input.trim();
    println!("Enter your name");
    println!("Nice to meet you {}", input);
} 