// interfaces
#[async_trait]
pub trait Store: Send + Sync {
    async fn store(&self, name: String);
    async fn get_users(&self) -> Vec<User>;
}

pub trait Delivery {
    fn create(&self);
}

// end interfaces

//models
pub struct User {
    pub name: String
}
// end models

//

pub struct Usecase {
    store: Box<dyn Store>,
}


impl Usecase {
    pub async fn create(&self, name: String)  {
        println!("usecase impl name: {}", name);
        self.store.store(name).await;
    }

    pub async fn get_users(&self) -> Vec<User> {
        self.store.get_users().await
    }
}

#[derive(Debug)]
pub enum BuisnessError {
    // Missing,
}

pub fn new_buisness(store: Box<dyn Store>) -> Usecase {
    return Usecase { store: store };
}
