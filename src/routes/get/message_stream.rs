use rocket::serde::json::Json;
use rocket::State;

use mutual_types::Message;

use crate::AppState;

#[get("/message_stream?<session_id>")]
pub async fn message_stream(session_id: String, state: &State<AppState>) -> Json<Message> {
    trace!("Connection to message_stream endpoint");
    if !crate::database::users::session_id_exists(&session_id).await.unwrap() {
        return Json(Message {
            author: "".to_string(),
            content: "Invalid session id".to_string(),
            timestamp: "".to_string(),
            channel: "".to_string(),
        });
    }
    Json::from(
        serde_json::from_str::<Message>(
            &*state.message_recv.recv()
                .expect("Failed to receive message"))
            .expect("Failed to parse message")
    )
}