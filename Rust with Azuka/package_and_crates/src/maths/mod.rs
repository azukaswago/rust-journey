pub use advanced::multiply as mul;
#[derive(Debug)]
pub enum DivideError {
    CannotDivideByZero,
    NegativeNumber,
}

///this is a simple function that adds two numbers together
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

///this is a simple function that subtracts two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

///this is a simple function that divides two numbers
pub fn divide(a: i32, b: i32) -> Result<i32, DivideError> {
    if a < 0 || b < 0 {
        Err(DivideError::NegativeNumber)
    } else if b == 0 {
        Err(DivideError::CannotDivideByZero)
    } else {
        Ok(a / b)
    }
}

pub mod advanced {
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn square_root(a: f64) -> f64 {
        a.sqrt()
    }

    pub fn modular(a: i32, b: i32) -> i32 {
        a % b
    }
}
