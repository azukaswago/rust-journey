#[derive(Debug, Clone)]
pub struct Expense {
    pub name: String,
    pub amount: f64,
    pub category: String,
}

impl Expense {
   pub  fn new(name: String, amount: f64, category: String) -> Self {
        Self { name, amount, category }
    }

    pub fn display(&self){
        println!("Name:{}, Cost:{}, Expense category:{}", self.name, self.amount, self.category)
    }
}