// // fn main() {
// //     let prices_in_usd = vec![98.0, 89_000.0, 2350.0];

// //     let prices_in_ngn: Vec<f64> = prices_in_usd.iter()
// //                                     .map(|price|*price * 1500.0)
// //                                     .collect();
// //     for price in prices_in_ngn {
// //         println!("#{}", price)
// //     };
// // }

// // // Each price goes into the closure, gets multiplied by 2, and comes out the other side. .collect() at the end pulls the iterator back into a Vec.
// // // The original prices vec is untouched — .map() produces a brand new collection.
// // // Exercise — you have a vec of BTC prices in USD. Use .map() to convert each price to NGN by multiplying by 1500. Collect into a new Vec<f64> and print it.

// struct Wallet {
//         owner: String,
//         balance: f64,
//     }
// fn main(){
//    let wallet_one = Wallet {
//         owner: String::from("Azuka"),
//         balance: 90.8,
//     };

//     let wallet_two = Wallet {
//         owner: String::from("Anayah"),
//         balance: 190.8,
//     };

//     let wallet_three = Wallet {
//         owner: String::from("Bale"),
//         balance: 80.8,
//     };

//     let wallets = vec![wallet_one, wallet_two, wallet_three];

//     let wallet_info: Vec<String> = wallets.iter().map(
//         |wallet| format!("{} holds {}", wallet.owner, wallet.balance)
//     ).collect();

//     for info in &wallet_info {
//         println!("{}", info)
//     };
// // Exercise — define a Wallet struct with owner: String and balance: f64.
// // Create a vec of 3 wallets. Use .map() to produce a Vec<String> where each string says "owner: balance NGN". Print it.//

// }

// Exercise — create a vec of 5 transaction amounts f64. Use .filter() to collect all amounts above 300.0 into a new vec and print it.
// Then use .find() to get the first amount above 300.0 and print that too.

// fn main() {
//     let transactions = vec![300.0, 150.0, 450.0, 600.0, 750.0];

//     let frivolities: Vec<&f64> = transactions.iter().filter(|transaction|**transaction > 300.0).collect();
//     println!("{:?}", frivolities);

//     let opportunity_cost = transactions.iter().find(|transaction|**transaction > 300.0);
//     match opportunity_cost {
//         Some(s) => println!("{} is the most viable opportunity cost", s),
//         None => println!("Create scale of preference"),
//     }
// }

// fn main(){
//     let split_transactions = vec![
//                                                 vec![90.0, 80.9, 87.8],
//                                                 vec![920.0, 180.9, 67.8],
//                                                 vec![90.0, 80.9,87.8],
//     ];
//     let merged_transactions:Vec<f64> = split_transactions.iter().flatten().cloned().collect();
//     println!("{:?}", merged_transactions);
// }

// create a vec of 5 transaction amounts f64.
// Use .partition() to split them into profitable (above 500.0) and unprofitable. Print both vecs.

// fn main() {
//     let transactions = vec![300.0, 150.0, 450.0, 600.0, 750.0];
//     let entry = 200.0;

//     let (profitable, non_profitable): (Vec<f64>, Vec<f64>) = transactions
//         .into_iter()
//         .partition(|transaction| *transaction > entry);

//     println!("{:?}", profitable);
//     println!("{:?}", non_profitable);
// }


//  create a vec of 3 wallet owners and a vec of 3 balances f64. Use .zip() to pair them up and print each owner with their balance.

// fn main (){
//     let owners = vec!["Azuka", "Basira", "Casalia"];
//     let balances = vec![9789000.0, 670.0, 540.0];

//     let zipped: Vec<(&&str, &f64)> = owners.iter().zip(balances.iter()).collect();

//     for (owner, balance) in &zipped {
//         println!("{} has ${}", owner, balance)
//     }
// }


// Exercise — create a vec of 4 transaction amounts f64. Use .fold() to calculate the total. Print it.
// fn main() {
//     let transactions = vec![9000.0, 78.0, 50.0, 11.0, -890.0];
//     let start_may11 = 900_000.0;

//     let end_may11 = transactions.iter().fold(start_may11, |balance, transaction| *transaction + balance);

//     println!("{}", end_may11);
// }


// Do the exercise — vec of 5 f64 transactions, print sum, count, highest and lowest.

// fn main(){
//     let transactions = vec![45.0, 76.9, 87.4, 65.0, 1.3];

//     let sum:f64 = transactions.iter().sum();
//     println!("{}", sum);

//     let count = transactions.iter().count();
//     println!("{}", count);

//     let highest = transactions.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
//     match highest {
//         Some(s) => println!("Highest: {}", s),
//         None => println!("None"),
//     };

    
//     let lowest = transactions.iter().min_by(|a, b| a.partial_cmp(b).unwrap());
//     match lowest {
//         Some(s) => println!("Lowest: {}", s),
//         None => println!("None"),
//     };
// }

// Exercise — create a vec of 5 token names. Print the last one, the element at index 2, the second from the back, and the position of any token of your choice.

// fn main(){
//     let tokens = vec!["SOL", "NEAR", "SUI", "DOT", "ADA"];

//     let last = tokens.iter().last();
//     match last {
//         Some(s) => println!("Last: {}", s),
//         None => println!("None"),
//     };

//     let index_two = tokens.iter().nth(2);
//      match index_two {
//         Some(s) => println!("At index two is: {}", s),
//         None => println!("None"),
//     };


//     let index_backtwo = tokens.iter().nth_back(2);
//      match index_backtwo {
//         Some(s) => println!("At index two is: {}", s),
//         None => println!("None"),
//     };


//     let choice = tokens.iter().position(|c|*c== "SUI");
//     match choice {
//         Some(s) => println!("My choice is: {}", s),
//         None => println!("None"),
//     };
// }

// Now do the exercise — vec of 6 transaction amounts, print the first 3, skip the first 2, reverse the whole vec, and take every 2nd element.

// fn main(){
//     let transactions = vec![76.0, 34.8, 8798.0, 78.9, 99.0, 123.0];

//     let abc: Vec<&f64> = transactions.iter().take(3).collect();
//     println!("{:?}", abc);

//     let skipidi_sigma: Vec<&f64> = transactions.iter().skip(2).collect();
//     println!("{:?}", skipidi_sigma);

//     let reverso: Vec<&f64> = transactions.iter().rev().collect();
//     println!("{:?}", reverso);

//     let skipped: Vec<&f64> = transactions.iter().step_by(2).collect();
//     println!("{:?}", skipped);
// }

// fn main() {
//     let mut  tnxs = vec![500.0, 760.0, 390.0, 230.0];
//     tnxs.sort_by(|a, b|a.partial_cmp(b).unwrap());
//     println!("{:?}", tnxs);
// }


use std::env;
use std::fs;
use std::path;

// fn main() {
//     let args: Vec<String> = env::args().skip(1).collect();
//     println!("{:?}", args);
// }

fn main(){
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("No token entered");
        return;
    }

    let  token = &args[0];
    let amount = match args.get(1) {
        Some(s) => s.parse::<f64>().unwrap_or(0.0),
        None => 0.0,
    };

    println!("Buying ${} worth of {}", amount, token);

    let path = fs::read_dir("./././").unwrap();
    for entry in path {
        match entry {
            Ok(path) => println!("{:?}", path.path()),
            Err(_e) => println!("File not found"),
        }
};

}