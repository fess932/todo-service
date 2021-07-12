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
        .mount("/", routes![delivery::count])
}
