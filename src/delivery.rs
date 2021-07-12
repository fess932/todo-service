use rocket::State;

use crate::buisness::{Buisness, Usecase};

#[get("/count")]
pub fn count(uscase: &State<Usecase>) -> String {
    let current_count = 2;
    uscase.create(String::from("lel"));
    format!("Number of visits: {}", current_count)
}
