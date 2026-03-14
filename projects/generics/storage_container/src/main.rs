use std::fmt::Debug;

#[derive(Debug)]
enum StorageType {
    Dry,
    Cold,
    Secure,
}

#[derive(Debug)]
struct Container<T, U> {
    item: T,
    metadata: U,
    storage_type: StorageType,
    capacity: u32,
}

impl<T, U: Debug> Container<T, U> {
    fn new(item: T, metadata: U, storage_type: StorageType, capacity: u32) -> Container<T, U> {
        Container {
            item,
            metadata,
            storage_type,
            capacity,
        }
    }

    fn get_capacity(&self) -> u32 {
        self.capacity
    }

    fn display_metadata(&self) {
        println!("Metadata:{:?}", self.metadata)
    }
}

impl Container<String, StorageType> {
    fn special_storage_report(&self) {
        println!(
            "Storing {} in {:?} with {:?} classification",
            self.item, self.storage_type, self.metadata
        )
    }
}
fn main() {
    let gold = Container::<i32, &str> {
        item: 15,
        metadata: "pot of gold",
        storage_type: StorageType::Secure,
        capacity: 300,
    };

    let meat = Container::<String, bool> {
        item: String::from("Mummy Chidera's pot of meat"),
        metadata: true,
        storage_type: StorageType::Cold,
        capacity: 25,
    };

    let soybeans = Container::<String, StorageType> {
        item: String::from("Soybeans powder"),
        metadata: StorageType::Dry,
        storage_type: StorageType::Dry,
        capacity: 900,
    };

    println!("Gold capacity: {}", gold.get_capacity());
    println!("Meat capacity: {}", meat.get_capacity());
    println!("Soybeans capacity: {}", soybeans.get_capacity());

    gold.display_metadata();
    meat.display_metadata();
    soybeans.display_metadata();

    soybeans.special_storage_report();
}
