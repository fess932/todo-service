use rocket::data::{Data, ToByteUnit};
use rocket::fairing::{Fairing, Info};
use rocket::http::{Method, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::route::{Handler, Outcome, Route};
use rocket::tokio;
use rocket::State;

use crate::buisness::{Buisness, BuisnessError, Usecase};

// pub struct Delivery {
//     usecase: Usecase,
// }

#[get("/count")]
pub fn count(uscase: &State<Usecase>) -> String {
    let current_count = 2;
    format!("Number of visits: {}", current_count)
}

// #[rocket::async_trait]
// impl Handler for Delivery {
//     async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
//         req.Me
//         match self.kind {
//             Kind::Simple => Outcome::from(req, "simple"),
//             Kind::Intermediate => Outcome::from(req, "intermediate"),
//             Kind::Complex => Outcome::from(req, "complex"),
//         }
//     }
// }

// impl Into<Vec<Route>> for Delivery {
//     fn into(self) -> Vec<Route> {
//         vec![Route::new(Method::Get, "/user", self)]
//     }
// }

// pub fn NewDelivery(u: Usecase, k: Kind) -> Delivery {
//     return Delivery {
//         usecase: u,
//         kind: k,
//     };
// }

// #[get("/user")]
// pub fn users(usecase: Usecase) {
//     usecase.create(String::from("lel"))
// }

// #[rocket::async_trait]
// impl<'a, 'r> FromRequest<'a, 'r> for Usecase {
//     type Error = ();

//     async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//         fn is_valid(key: &str) -> bool {
//             key == "valid_api_key"
//         }

//         match request.headers().get_one("x-api-key") {
//             None => Outcome::Failure((Status::BadRequest, BuisnessError::Missing)),
//             Some(key) if is_valid(key) => Outcome::Success(Usecase {}),
//             Some(_) => Outcome::Failure((Status::BadRequest, BuisnessError::Missing)),
//         }
//     }
// }

// impl Fairing for Delivery {
//     fn info(&self) -> Info {
//         Info {
//             name: "DELIVERY FOR BUISNESS LOGIC",
//             kind: Kind::Request | Kind::Response,
//         }
//     }
// }

// impl buisness::Delivery for Delivery {
//     fn create(&self) {}
// }

// #[post("/user/create", data = "<data>")]
// pub async fn create_user(data: Data<'_>) -> std::io::Result<()> {
//     Delivery.create();
//     Ok(())
// }

// #[post("/hello", data = "<data>")]
// pub async fn hello_world(data: Data<'_>) -> std::io::Result<()> {
//     data.open(512.kibibytes())
//         .stream_to(tokio::io::stdout())
//         .await?;
//     Ok(())
// }
