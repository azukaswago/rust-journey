mod maths;
use::rand::Rng; 
use maths::advanced::*;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn main() {
    let a = 3;
    let b = 5;
    let resulting = maths::add(a, b);
    println!("{}", resulting);

    let n = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", n);


    let c = 0;
    let d = 4;
    let e = -1;

    let pointer = Point { x: 1.0, y: 2.0 };
    println!("{:?}", pointer);
    println!("x: {}, y: {}", pointer.x, pointer.y);

    let resulting_minus = maths::subtract(d, e);
    println!("{}", resulting_minus);

    match maths::divide(d, e) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e),
    }

    match maths::divide(d, c) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e),
    }

    match maths::divide(d, a) {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{:?}", e),
    }

    let resulting_multiply = maths::mul(d, e);
    println!("{}", resulting_multiply);

    let resulting_mod = modular(d, e);
    println!("{}", resulting_mod);

    let resulting_sqrt = maths::advanced::square_root(d as f64 * 121 as f64);
    println!("{}", resulting_sqrt);
}
