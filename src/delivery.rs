use rocket::State;

use crate::buisness::{Usecase};

#[get("/user/<name>")]
pub async fn count(uscase: &State<Usecase>, name: String) -> String {
    let current_count = 2;
    
    uscase.create(name).await;

    format!("Number of visits: {}", current_count)
}
