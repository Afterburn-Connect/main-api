use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use afterburn_types::Message;

use crate::database::messages::send_message;
use crate::AppState;

#[post("/message/<session>", data = "<data>", format = "json")]
pub async fn message(data: Json<Message>, session: String, state: &State<AppState>) -> Status {
    trace!("Connection to message endpoint");
    match send_message(data.clone().into_inner(), session, state).await {
        Ok(stat_code) => Status::new(stat_code as u16),
        Err(err) => {
            eprintln!("{:?}", err); // Print the error to the console
            Status::BadRequest
        }
    }
}
