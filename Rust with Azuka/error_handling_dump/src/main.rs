// // // #[derive(Debug)]
// // // struct Food {
// // //     name: String,
// // // }
// // // #[derive(Debug)]
// // // struct Restaurant {
// // //     reservations: u32,
// // //     has_mice_infestation: bool,
// // // }
// // // impl Restaurant {
// // //     fn chef_special(&self) -> Option<Food> {
// // //         if self.has_mice_infestation {
// // //             None
// // //         } else if self.reservations < 12 {
// // //             Some(Food { name: String::from("Uni Sashimi") })
// // //         } else {
// // //             Some(Food { name: String::from("Strip Steak") })
// // //         }
// // //     }
// // //     fn deliver_burger(&self, address: &str) -> Result<Food, String> {
// // //         if self.has_mice_infestation {
// // //             Err(String::from("Sorry, we have a mice problem"))
// // //         } else if address.is_empty() {
// // //             Err(String::from("No delivery address specified"))
// // //         } else {
// // //             Ok(Food { name: String::from("Burger") })
// // //         }
// // //     }
// // // }
// // // fn main() {
// // //     let first_instance = Restaurant {
// // //         reservations: 11,
// // //         has_mice_infestation: true,
// // //     };
// // //     let chef_special = first_instance.chef_special();
// // //     let deliver_burger = first_instance.deliver_burger("123 Elm Street");

// // //     let second_instance = Restaurant {
// // //         reservations: 15,
// // //         has_mice_infestation: false,
// // //     };
// // //     let chef_special_two = second_instance.chef_special();
// // //     let address = "";
// // //     let deliver_burger1 = second_instance.deliver_burger(address);
// // //     let addy1 = "Liverpool Estate, Zone 2, Satellite Town";
// // //     let deliver_burger2 = second_instance.deliver_burger(addy1);

// // //     println!("--- Restaurant 1 ---");
// // //     match chef_special {
// // //         Some(s) => println!("Chef special: {}", s.name),
// // //         None => println!("There is no chef special!"),
// // //     }
// // //     match deliver_burger {
// // //         Ok(n) => println!("Delivering {} to 123 Elm Street", n.name),
// // //         Err(e) => println!("Delivery error: {}", e),
// // //     }

// // //     println!("--- Restaurant 2 ---");
// // //     match chef_special_two {
// // //         Some(s) => println!("Chef special: {}", s.name),
// // //         None => println!("There is no chef special!"),
// // //     }
// // //     match deliver_burger1 {
// // //         Ok(n) => println!("Delivering {} to {}", n.name, address),
// // //         Err(e) => println!("Delivery error: {}", e),
// // //     }
// // //     match deliver_burger2 {
// // //         Ok(n) => println!("Delivering {} to {}", n.name, addy1),
// // //         Err(e) => println!("Delivery error: {}", e),
// // //     }
// // // }

// // // use std::process;

// // // fn test_eprint(a:i32, b:i32)->i32{
// // //     if b == 0 {
// // //         eprintln!("Cannot divide by zero");
// // //       process::exit(1)
// // // }
// // // else {
// // //     a/b
// // // }
// // // }

// // // fn main(){
// // //     let a = 3;
// // //     let b = 0;

// // //     test_eprint(a, b);
// // // }
 
// // // use std::fs::File;
// // // use std::fs;

// // // fn open_file(path: &str) {
// // //     match File::open(path) {
// // //         Ok(_) => println!("file opened succesfully"),
// // //         Err(e)=> eprintln!("File not found! {}", e),
// // //     }
// // // }

// // // fn read_file(path: &str) {
// // //     match fs::read_to_string(path) {
// // //         Ok(contents) => println!("{}", contents),
// // //         Err(e)=> eprintln!("{}", e),
// // //     }
// // // }
// // // fn main(){
// // //     open_file("hello.txt");
// // //     read_file("hello.txt");
// // // }

// // type IoError = std::io::Error;
// // use std::fs;
// // fn get_file_contents(path:&str)-> Result<String,IoError> {
// //     let contents = fs::read_to_string(path)?;
// //     Ok(contents)
// // }

// // fn get_vector(v: &Vec<i32>) -> Option<&i32> {
// //     let first = v.first() ?;
// //     Some(*&first)
// // }

// // fn main(){
// //     match get_file_contents("hello.txt") {
// //         Ok(contents) => println!("{}", contents),
// //         Err(e)=> println!("{}", e),
// //     }

// //     let v = Vec::new();
// //     let mut u = v.clone();
// //     u.push(3);
// //     u.push(7);
// //     u.push(5);
// //     u.push(9);

// //     match get_vector(&v) {
// //         Some(s) => println!("{}", s),
// //         None => println!("NO VALUE"),
// //     }

// //     match get_vector(&u) {
// //         Some(s) => println!("{}", s),
// //         None => println!("NO VALUE"),
// //     }
// // }

// use std::fmt;

// #[derive(Debug)]
// enum AppError {
//     DivisionByZero,
//     NegativeNumber,
// }

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             AppError::DivisionByZero => write!(f, "Cannot divide by zero"),
//             AppError::NegativeNumber => write!(f, "Negative numbers not allowed"),
//         }
//     }
// }


// fn safe_divide(a:i32, b:i32) -> Result<i32, AppError> {
//     if b == 0 {
//         Err(AppError::DivisionByZero)
//     }
//     else if a < 0  || b < 0 {
//             Err(AppError::NegativeNumber)
//         }
//     else {
//         Ok(a/b)
//     }
// }
// fn main() {
//     let a = 3;
//     let b = 4;
//     let c = 0;
//     let d = -2;

//     match safe_divide(a, b) {
//         Ok(answer) => println!("{}", answer),
//         Err(e) => println!("{}", e),
//     }

//     match safe_divide(a, c) {
//         Ok(answer) => println!("{}", answer),
//         Err(e) => println!("{}", e),
//     }

//     match safe_divide(a, d) {
//         Ok(answer) => println!("{}", answer),
//         Err(e) => println!("{}", e),
//     }
// }


use std::fs::File;
use std::io::{Error, Read};
use std::io;

fn main(){
   let mut file =  File::open("hello.txt").expect("couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read the contents of this file");
    println!("{}", contents);
    println!("\n");

let mut info = String::new();
println!("What did you gain from this article?");
println!("\n");
io::stdin().read_line(&mut info).expect("invalid message");
println!("\n");
println!("Thanks for your feedback");

match filing("hello.txt") {
    Ok(contents) => println!("{}", contents),
    Err(e) => println!("Error: {}", e),
}

fn filing(path:&str)-> Result<String, io::Error> {
   std::fs::read_to_string("hello.txt")
}
}

fn vector(pop: Vec<Vec<i32>) -> Option<i32> {
    
}