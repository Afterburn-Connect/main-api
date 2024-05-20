extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rocket;

use std::process::exit;

use port_scanner::scan_port_addr;
use rocket::http::Method;

use crate::config::get_config;
use crate::routes::{get, post};
use redis::Client;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod config;
pub mod database;
pub mod routes;

pub struct AppState {
    pub redis_client: Client,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app_state = AppState {
        redis_client: Client::open(get_config().redis_url).unwrap(),
    };

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allowed_methods: vec![Method::Post, Method::Get]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    if !scan_port_addr(get_config().surreal_host) {
        eprintln!("Please start SurrealDB with the docker compose file");
        exit(1);
    }
    if !scan_port_addr(get_config().redis_host) {
        eprintln!("Please start Redis with the docker compose file");
        exit(1);
    }
    rocket::build()
        .manage(app_state)
        .mount(
            "/",
            routes![
                post::login::login,
                post::signup::signup,
                post::message::message,
                get::status::status,
                get::messages::messages_since
            ],
        )
        .attach(cors)
        .ignite()
        .await
        .unwrap()
        .launch()
        .await
        .expect("Web server failed to start");
}
