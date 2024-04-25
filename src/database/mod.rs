use surrealdb::{Error, Surreal};
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;

use crate::config::get_config;

pub mod users;
mod channels;
pub mod messages;


pub async fn connect() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Http>(get_config().surreal_host.as_str()).await?;

    db.signin(Root {
        username: get_config().surreal_username.as_str(),
        password: get_config().surreal_password.as_str(),
    })
        .await?;

    // Select a specific namespace / database
    db.use_ns("debate").use_db(get_config().surreal_db).await?;

    Ok(db)
}