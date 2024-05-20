use rand::Rng;

use afterburn_types::User;

use crate::database::connect;

/// Generate a 32 character random hex string.
///
/// This function generates a random session ID, which is a 32 character long
/// hexadecimal string. It uses the `rand` crate to generate random numbers,
/// which are then formatted as hexadecimal and collected into a string.

pub async fn gen_random_session_id() -> String {
    let mut rng = rand::thread_rng();
    (0..32).map(|_| format!("{:X}", rng.gen::<u8>())).collect()
}

/// Get a session token for a user.
///
/// This function retrieves a session token for a user identified by their UUID
/// and password. If the user is found in the database, a new session is created
/// and the session ID is returned. If the user is not found, an empty string is
/// returned.
pub async fn get_session_token(email: &str, password: &str) -> Result<String, surrealdb::Error> {
    trace!(
        "Attempting sessions token generation for {}:{}",
        email,
        password
    );
    let conn = connect().await?;

    let mut query = "SELECT * FROM users WHERE email = $email AND password = $password";

    let mut result = conn
        .query(query)
        .bind(("email", email))
        .bind(("password", password))
        .await?;
    let result = result.take::<Vec<User>>(0)?;
    return if result.len() == 1 {
        trace!(
            "Email and password combination valid for {}:{}",
            email,
            password
        );
        let uuid = user_from_email(email).await?.unwrap().uuid;
        query = "DELETE FROM sessions WHERE uuid = $uuid";
        conn.query(query).bind(("uuid", &uuid)).await?;
        let session_id = gen_random_session_id().await;
        trace!(
            "Generated session ID: {} for {}:{}",
            session_id,
            email,
            password
        );
        let session_id = session_id.as_str();
        query = "INSERT INTO sessions (session_id, uuid) VALUES ($session_id, $uuid)";
        conn.query(query)
            .bind(("session_id", session_id))
            .bind(("uuid", uuid))
            .await?;
        trace!("Session ID inserted into database");
        Ok(session_id.to_string())
    } else if result.len() > 1 {
        trace!("Duplicate users found for {}:{}", email, password);
        Ok("".to_string())
    } else {
        trace!(
            "Email and password combination not valid for {}:{}",
            email,
            password
        );
        Ok("".to_string())
    };
}

/// Get a user from their UUID.
///
/// This function retrieves a user from the database using their UUID. If the
/// user is found, they are returned. If the user is not found, `None` is
/// returned.
pub async fn user_from_uuid(uuid: &str) -> Result<Option<User>, surrealdb::Error> {
    trace!("Attempting to retrieve user with UUID: {}", uuid);
    let conn = connect().await?;

    let query = "SELECT * FROM users WHERE uuid = $uuid";

    let mut result = conn
        .query(query)
        .bind(("uuid", uuid))
        .await?
        .take::<Vec<User>>(0)?;
    if result.len() == 1 {
        match result.pop() {
            Some(user) => {
                trace!("UUID {} found: {}", uuid, user.username);
                Ok(Some(user))
            }
            None => {
                trace!("UUID {} not found", uuid);
                Ok(None)
            }
        }
    } else {
        trace!("UUID {} not found", uuid);
        Ok(None)
    }
}

/// Get a user from their session ID.
///
/// This function retrieves a user from the database using their session ID. If
/// the user is found, they are returned. If the user is not found, `None` is
/// returned.
pub async fn user_from_session_id(session_id: &str) -> Result<Option<User>, surrealdb::Error> {
    trace!(
        "Attempting to retrieve user with session ID: {}",
        session_id
    );
    let conn = connect().await?;

    let query = "SELECT VALUE uuid FROM sessions WHERE session_id = $session_id";

    let result = conn
        .query(query)
        .bind(("session_id", session_id))
        .await?
        .take::<Option<String>>(0)?;
    if result.is_none() {
        return Ok(None);
    }
    let uuid = result.unwrap();
    trace!("Session ID {} found: {}", session_id, uuid);
    user_from_uuid(uuid.as_str()).await
}

pub async fn user_from_username(username: &str) -> Result<Option<User>, surrealdb::Error> {
    trace!("Attempting to retrieve user with username: {}", username);
    let conn = connect().await?;

    let query = "SELECT * FROM users WHERE username = $username";

    let mut result = conn
        .query(query)
        .bind(("username", username))
        .await?
        .take::<Vec<User>>(0)?;
    if result.len() == 1 {
        match result.pop() {
            Some(user) => {
                trace!("Username {} found: {}", username, user.username);
                Ok(Some(user))
            }
            None => {
                trace!("Username {} not found", username);
                Ok(None)
            }
        }
    } else {
        trace!("Username {} not found", username);
        Ok(None)
    }
}

pub async fn user_from_email(email: &str) -> Result<Option<User>, surrealdb::Error> {
    trace!("Attempting to retrieve user with email: {}", email);
    let conn = connect().await?;

    let query = "SELECT * FROM users WHERE email = $email";

    let mut result = conn
        .query(query)
        .bind(("email", email))
        .await?
        .take::<Vec<User>>(0)?;
    if result.len() == 1 {
        match result.pop() {
            Some(user) => {
                trace!("Email {} found: {}", email, user.username);
                Ok(Some(user))
            }
            None => {
                trace!("Email {} not found", email);
                Ok(None)
            }
        }
    } else {
        trace!("Email {} not found", email);
        Ok(None)
    }
}

/// Validate a session ID.
///
/// This function validates a session ID by checking if it exists in the
/// database and is associated with the provided UUID. If the session ID is
/// valid, `true` is returned. If the session ID is not valid, `false` is
/// returned.
pub async fn validate_session_id(session_id: &str, uuid: String) -> Result<bool, surrealdb::Error> {
    let conn = connect().await?;
    trace!("Validating session ID: {}", session_id);

    let query =
        "SELECT VALUE session_id FROM sessions WHERE session_id = $session_id AND uuid = $uuid";

    let result: Option<String> = conn
        .query(query)
        .bind(("session_id", session_id))
        .bind(("uuid", uuid))
        .await?
        .take(0)?;

    if result.is_some() {
        trace!("Session ID {} is valid", session_id);
        Ok(true)
    } else {
        trace!("Session ID {} is not valid", session_id);
        Ok(false)
    }
}

pub async fn session_id_exists(session_id: &str) -> Result<bool, surrealdb::Error> {
    let conn = connect().await?;
    trace!("Checking if session ID exists: {}", session_id);

    let query = "SELECT VALUE session_id FROM sessions WHERE session_id = $session_id";

    let result: Option<String> = conn
        .query(query)
        .bind(("session_id", session_id))
        .await?
        .take(0)?;

    if result.is_some() {
        trace!("Session ID {} exists", session_id);
        Ok(true)
    } else {
        trace!("Session ID {} does not exist", session_id);
        Ok(false)
    }
}

/// Sign up a new user.
///
/// This function creates a new user in the database. The user's UUID is
/// generated automatically, and their profile picture and bio are set to empty
/// strings. If a user with the same email already exists in the database, the
/// function returns `405`. If the user is created successfully, the function
/// returns `200`.
pub async fn sign_up(user: User) -> Result<u32, surrealdb::Error> {
    let conn = connect().await?;
    trace!("Attempting to sign up user: {}", user.username);

    let newuser = User {
        uuid: uuid::Uuid::new_v4().to_string(),
        pfp: "".to_string(),
        bio: "".to_string(),
        password: user.password,
        ..user
    };

    let mut result = conn
        .query("SELECT * FROM users WHERE email = $email")
        .bind(("email", &newuser.email))
        .await?;

    return if result.take::<Vec<User>>(0)?.len() > 0 {
        trace!("Email {} already exists", &newuser.email);
        Ok(405)
    } else {
        conn.query("INSERT INTO users $user")
            .bind(("user", &newuser))
            .await?;
        trace!("User {} signed up", newuser.username);
        Ok(200)
    };
}
