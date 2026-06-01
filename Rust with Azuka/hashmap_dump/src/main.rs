// use std::collections::{HashMap, HashSet};
// use std::io;

// fn main() {
//     let eth = "Ethereum";
//     let sol = "Solana";
//     let btc = "Bitcoin";
//     let okz = "Okeosisi";

//     let mut market = HashMap::new();
//     market.insert(sol, 180.75);
//     market.insert(btc, 100090.75);
//     market.insert(eth, 2080.75);

//     println!("{:?}", market);

//     market.remove(eth);
//     println!("{:?}", market);

//     market.insert(sol, 80.0);
    
//     match market.get(sol) {
//         Some(s) => println!("After the recent war crises, Solana crashed to ${}",s),
//         None => println!("Missing")
//     }

//      match market.get(okz) {
//         Some(s) => println!("Azuka launched his utility at  ${}", s),
//         None => println!("Okeosisi hasn't been launched"),
//     }
    
//     market.entry(okz).or_insert(23.02);

//     match market.get(okz) {
//         Some(s) => println!("After the X space held by the promising ZK engineer, his utility coin launched at ${}", s),
//         None => println!("Still hasn't launched"),
//     }

//     let mut input = String::new();
//     println!("Enter name of crypto you want to look up price of");
//     io::stdin().read_line(&mut input).expect("invalid key");
//     println!("The price of {} is ${}", input.trim(), market.get(input.trim()).unwrap_or(&0.0));


//    let mut dupe = HashSet::new();
//     dupe.insert(sol);
//     dupe.insert(btc);
//     dupe.insert(eth);
//      dupe.insert(btc);
//     dupe.insert(eth);

//     println!("{:?}", dupe);
   
// }

    /*
    Bring the HashMap type into the current's file's namespace.
     
    Declare a `sauces_to_meals` HashMap. The keys will be
    string slices and the values will be a vector of string
    slices. Use the `from` function to populate the HashMap
    with 2 key-value pairs:
     
    Key: "Ketchup"
    Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]
     
    Key: "Mayonnaise"
    Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]
     
    Use the `insert` method to add the following key-value
    pair to the HashMap.
     
    Key: "Mustard"
    Value: Vector of ["Hot dog", "Burgers", "Pretzels"]
     
    Use the `remove` method to remove the key-value pair
    where "Mayonnaise" is the key. Find a way to retrieve
    the vector inside the Option and print it out.
     
    Use the `get` method to retrieve the key-value pair
    where "Mustard" is the key. Find a way to retrieve
    the vector inside the Option and print it out.
     
    Use the `entry` and `or_insert` methods to add the
    following key-value pair:
     
    Key: "Soy Sauce"
    Value: Vector of ["Sushi", "Dumplings"]
     
    Finally, print out the final `sauces_to_meals` HashMap.
     
    The final result should be:
    {
      "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
      "Soy Sauce": ["Sushi", "Dumplings"],
      "Mustard": ["Hot dog", "Burgers", "Pretzels"]
    }
    */
    use std::collections::HashMap;
    fn main() {
   let mut sauces_to_meals: HashMap<&str ,Vec<&str>> = HashMap::new();
   sauces_to_meals.insert("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]);
   sauces_to_meals.insert("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]);
   sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretzels"]);

   match sauces_to_meals.remove("Mayonnaise") {
    Some(ingredients) => println!("{:?}", ingredients),
    None => println!("Mayo not found"),
   }

   match sauces_to_meals.get("Mustard") {
    Some(ingredients) => println!("{:?}", ingredients),
    None => println!("Mustard is not on the beat ho"),
   }

   sauces_to_meals.entry("Soy Sauce").or_insert(vec!["Sushi", "Dumplings"]);
   println!("{:?}", sauces_to_meals);

}
