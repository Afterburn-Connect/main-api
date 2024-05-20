use crate::database::{connect, users};
use afterburn_types::Message;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/messages_since/<session>/<timestamp>")]
pub async fn messages_since(session: String, timestamp: u64) -> Result<Json<Vec<Message>>, Status> {
    return match connect().await {
        Ok(conn) => match users::user_from_session_id(session.as_str()).await.unwrap() {
            Some(_) => {
                let query = "SELECT VALUE message FROM (SELECT * FROM events WHERE timestamp > $timestamp AND  event_type = 'Message' ORDER BY timestamp DESC);";
                let mut rows = conn
                    .query(query)
                    .bind(("timestamp", timestamp))
                    .await
                    .unwrap();

                let messages = rows.take(0).unwrap();

                Ok(Json(messages))
            }
            None => {
                return Err(Status::Unauthorized);
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Err(Status::InternalServerError)
        }
    };
}
