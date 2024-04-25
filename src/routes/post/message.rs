use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use mutual_types::Message;

use crate::AppState;
use crate::database::messages::send_message;

#[post("/message/<session>", data = "<data>", format = "json")]
pub async fn message(data: Json<Message>, session: String, state: &State<AppState>) -> Status {
    trace!("Connection to message endpoint");
    match send_message(data.clone().into_inner(), session).await {
        Ok(stat_code) => {
            state.message_send.send(serde_json::to_string(&data.into_inner()).unwrap()).expect("Failed to send message to channel");
            Status::new(stat_code as u16)
        }
        Err(err) => {
            eprintln!("{:?}", err); // Print the error to the console
            Status::BadRequest
        }
    }
}