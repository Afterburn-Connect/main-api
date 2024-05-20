use crate::AppState;
use afterburn_types::{Event, EventType, Message};
use chrono;
use redis::Commands;
use rocket::State;

use crate::database::connect;
use crate::database::users::validate_session_id;

pub async fn send_message(
    message: Message,
    session_id: String,
    state: &State<AppState>,
) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;
    trace!("Message received");

    let event = Event {
        event_type: EventType::Message,
        message: Some(message.clone()),
        user: Some(message.author.clone()),
        timestamp: chrono::Utc::now().timestamp().to_string(),
    };

    match state.redis_client.get_connection() {
        Ok(mut con) => {
            let _: () = con
                .publish("events", serde_json::to_string(&event).unwrap())
                .expect("Failed to publish event");
        }
        Err(e) => {
            error!("Error publishing event: {}", e);
            return Ok(500);
        }
    }

    if !validate_session_id(&session_id, message.author.clone())
        .await
        .unwrap()
    {
        trace!("Invalid session ID");
        return Ok(401);
    }

    conn.create::<Option<Event>>(("events", event.clone().timestamp))
        .content(event)
        .await?;
    trace!("Message sent");
    Ok(200)
}
