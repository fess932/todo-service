// interfaces
#[async_trait]
pub trait Store {
    async fn store(&self, name: String);
}

pub trait Delivery {
    fn create(&self);
}

// interfaces

pub struct Usecase {
    store: Box<dyn Store + Send + Sync>,
}

impl Usecase {
    pub async fn create(&self, name: String)  {
        println!("usecase impl name: {}", name);
        self.store.store(name).await;
    }
}

#[derive(Debug)]
pub enum BuisnessError {
    // Missing,
}

pub fn new_buisness(store: Box<dyn Store + Send + Sync>) -> Usecase {
    return Usecase { store: store };
}
