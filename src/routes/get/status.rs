use rocket::serde::json::Json;

use mutual_types::types::StatusResponse;

use crate::database::connect;

#[get("/status")]
pub async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        db: connect().await.is_ok(),
    })
}
