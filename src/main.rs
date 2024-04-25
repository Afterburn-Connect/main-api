extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate rocket;

use which::which;

use crate::config::get_config;
use crate::routes::{get, post};

mod config;
pub mod database;
pub mod routes;

pub struct AppState {
    pub message_recv: crossbeam_channel::Receiver<String>,
    pub message_send: crossbeam_channel::Sender<String>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (s, r) = crossbeam_channel::unbounded();
    let app_state = AppState {
        message_recv: r,
        message_send: s,
    };
    pretty_env_logger::env_logger::Builder::from_default_env()
        .filter_module("backend", log::LevelFilter::Trace)
        .init();

    match which("surreal") {
        Ok(_) => {}
        Err(_) => {
            println!("Please install surrealdb");
            std::process::exit(1);
        }
    }

    tokio::spawn(async {
        std::process::Command::new("surreal")
            .arg("start")
            .arg("file:database.db")
            .arg("--auth")
            .arg("--user")
            .arg(get_config().surreal_username)
            .arg("--pass")
            .arg(get_config().surreal_password)
            .arg("--bind")
            .arg("0.0.0.0:9000")
            .spawn()
            .unwrap()
            .wait()
    });
    rocket::build()
        .manage(app_state)
        .mount(
            "/",
            routes![
                post::login::login,
                post::signup::signup,
                post::message::message,
                get::message_stream::message_stream,
                get::status::status,
            ],
        )
        .ignite()
        .await
        .unwrap()
        .launch()
        .await
        .expect("Web server failed to start");
}
