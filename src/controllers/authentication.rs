use std::{
    env,
    fmt::Display,
    sync::Arc,
    time::{Duration, SystemTime},
};

use axum::{
    routing::post,
    Router,
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tower_http::add_extension::AddExtensionLayer;

use crate::controllers::auth::change_password::change_password;
use crate::controllers::auth::login::login;
use crate::controllers::auth::reset_password::reset_password;
use crate::controllers::auth::send_email::send_email;
use crate::controllers::auth::signup::signup;

#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
    pub(crate) people: Vec<User>,
    pub(crate) total_records: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub email: String,
    pub(crate) verification_code: Option<String>,
}

// add a new() function so the struct can be initialized if it doesn't exist
impl UserData {
    pub fn new() -> Self {
        Self {
            people: Vec::new(),
            total_records: 0,
        }
    }
}

pub struct MyState {
    pub(crate) persist: Arc<PersistInstance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    role: String,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {} Role: {}", self.sub, self.role)
    }
}

static SECRET_KEY: once_cell::sync::Lazy<jsonwebtoken::EncodingKey> =
    once_cell::sync::Lazy::new(|| {
        let secret = env::var("SECRET_KEY").unwrap_or_else(|_| panic!("SECRET_KEY must be set"));
        jsonwebtoken::EncodingKey::from_secret(secret.as_ref())
    });




pub async fn create_jwt(username: String) -> String {
    let claims = Claims {
        sub: username,
        role: "User".to_string(),
        exp: (SystemTime::now() + Duration::from_secs(24 * 60 * 60))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize,
    };

    // Encode the claims into a JWT
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &SECRET_KEY).unwrap()
}

pub(crate) fn validate_jwt(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| panic!("SECRET_KEY must be set"));
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn auth_routes(persist: PersistInstance) -> Router {
    let state = Arc::new(MyState {
        persist: Arc::new(persist),
    });

    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/send_email/:email", post(send_email))
        .route("/reset", post(reset_password))
        .route("/changepassword", post(change_password))
        .layer(AddExtensionLayer::new(state))
}
