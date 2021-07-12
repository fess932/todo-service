// interfaces
pub trait Store {
    fn store(&self, name: String);
}

pub trait Delivery {
    fn create(&self);
}

pub trait Buisness {
    fn create(&self, name: String);
}
// interfaces

pub struct Usecase {
    store: Box<dyn Store>,
}

impl Buisness for Usecase {
    fn create(&self, name: String) {
        println!("usecase impl");
    }
}

#[derive(Debug)]
pub enum BuisnessError {
    Missing,
}

pub fn NewBuisness(store: impl Store + 'static) -> Usecase {
    return Usecase {
        store: Box::new(store),
    };
}
