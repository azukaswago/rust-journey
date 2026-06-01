use nigeria::lagos::greet as eko ;
use nigeria::abuja::greet as fct;
use rand::Rng;

mod person {
    pub struct Person {
        pub name: String,
        pub age: u32,
    }
}

  mod nigeria {
        pub mod lagos {
          pub fn greet(name: String){
                println!("{}, eko o ni baje o",name)
            }
        }

        pub mod abuja {
            /// Greets a person by name
/// 
/// # Examples
/// ```
/// greet("Azuka");
/// ```
            pub fn greet(name: String) {
                println!("Welcome {} to the Federal Capital Territory", name)

            }
        }
    }
fn main() {
    let p = person::Person {
        name: String::from("Azuka"),
        age: 19,
    };
    println!("{} is {}", p.name, p.age);
    let name = "Azuka";

    eko(p.name.clone());

    let n: u32 = rand::thread_rng().gen_range(1..=10);
println!("{}", n);

}