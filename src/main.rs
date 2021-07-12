mod buisness;
mod delivery;
mod repository;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let repo = repository::NewStore().await.expect("wtf");

    // let buis = buisness::NewBuisness(repo);

    let s = State {
        count: 2,
        s: String::from("lel"),
        b: Box::new(Inside {}),
    };

    rocket::build()
        .manage(s)
        // .manage(buis)
        .mount("/", routes![delivery::count])
}

struct State {
    count: i32,
    s: String,
    b: Box<Inside>,
}

struct Inside {}
