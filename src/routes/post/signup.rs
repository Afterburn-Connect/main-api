use rocket::http::Status;
use rocket::serde::json::Json;

use mutual_types::User;

use crate::database;

#[post("/signup", data = "<data>", format = "json")]
pub async fn signup(data: Json<User>) -> Status {
    trace!("Connection to signup endpoint");
    match database::users::sign_up(data.into_inner()).await {
        Ok(stat_code) => Status::new(stat_code as u16),
        Err(_) => Status::BadRequest,
    }
}