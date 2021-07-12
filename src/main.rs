use rocket_dyn_templates::Template;

mod buisness;
mod delivery;
mod repository;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let repo = repository::new_store().await.expect("wtf");

    let buis = buisness::new_buisness(repo);

    rocket::build()
        .manage(buis)
        .attach(Template::fairing())
        .mount("/", routes![delivery::add_user, delivery::get_users])
}
