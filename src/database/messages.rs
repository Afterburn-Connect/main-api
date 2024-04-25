use mutual_types::Message;

use crate::database::connect;
use crate::database::users::validate_session_id;

pub async fn send_message(message: Message, session_id: String) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;
    trace!("Message received");

    if !validate_session_id(&session_id, message.author.clone()).await.unwrap() {
        trace!("Invalid session ID");
        return Ok(401);
    }

    let query = "CREATE messages CONTENT $message";

    conn.query(query).bind(("message", message)).await?;
    trace!("Message sent");
    Ok(200)
}