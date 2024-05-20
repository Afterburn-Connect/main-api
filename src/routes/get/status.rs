use rocket::serde::json::Json;

use crate::AppState;
use afterburn_types::types::StatusResponse;
use redis::ConnectionLike;
use rocket::State;
use sys_info;

use crate::database::connect;

#[get("/status")]
pub async fn status(state: &State<AppState>) -> Json<StatusResponse> {
    let redis_conn = state.redis_client.clone().check_connection();
    let cpu = sys_info::loadavg().unwrap().one;
    let mem = sys_info::mem_info().unwrap().free as f32
        / sys_info::mem_info().unwrap().total as f32
        * 100.0;
    Json(StatusResponse {
        db: connect().await.is_ok(),
        cpu: cpu as f32,
        mem: mem as u64,
        redis: redis_conn,
        tornado: false,
    })
}
