use rocket::State;
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::buisness::{Usecase};

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    users: Vec<HtmlUser>,
}

#[derive(Serialize)]
struct HtmlUser {
    name: String
}

#[get("/")]
pub async fn todo_list(uscase: &State<Usecase>) -> Template {
    println!("Number of visits");

    let mut u = TemplateContext{title: String::from("List of all users"), users: vec![]};
    let users = uscase.get_users().await;

    for x in &users {
        u.users.push(HtmlUser{name: x.name.clone()});
    }

    Template::render("main", u)
}

#[post("/user/<name>")]
pub async fn add_user(uscase: &State<Usecase>, name: String) -> String {
    uscase.create(name.clone()).await;
    format!("User created with name, {}", name)
}
