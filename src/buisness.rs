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
    store: Box<dyn Store + Send + Sync>,
}

impl Buisness for Usecase {
    fn create(&self, name: String) {
        println!("usecase impl name: {}", name);
        self.store.store(name);
    }
}

#[derive(Debug)]
pub enum BuisnessError {
    Missing,
}

pub fn new_buisness(store: Box<dyn Store + Send + Sync>) -> Usecase {
    return Usecase { store: store };
}
