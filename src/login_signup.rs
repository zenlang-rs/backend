use std::{
    fmt::Display,
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::login_signup::headers::authorization::Bearer;
use axum::{
    extract::{self, TypedHeader},
    headers,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use bcrypt::{hash, DEFAULT_COST};
use headers::Authorization;
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tower_http::add_extension::AddExtensionLayer;
// as you can see this allows for a nested structs
#[derive(Deserialize, Serialize)]
pub struct UserData {
    people: Vec<User>,
    total_records: i32,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    name: String,
    username: String,
    password: String,
    email: String,
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

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

struct MyState {
    persist: Arc<PersistInstance>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {} Role: {}", self.sub, self.role)
    }
}

static SECRET_KEY: once_cell::sync::Lazy<jsonwebtoken::EncodingKey> =
    once_cell::sync::Lazy::new(|| jsonwebtoken::EncodingKey::from_secret("zen-lang-org".as_ref()));

async fn signup(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<User>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if req.email.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Email cannot be empty".to_string()));
    } else if req.password.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Password cannot be empty".to_string(),
        ));
    } else if req.name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
    } else if req.username.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "UserName cannot be empty".to_string(),
        ));
    }
    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };
    // Check if a user with the same email already exists
    if data.people.iter().any(|person| person.email == req.email) {
        return Err((
            StatusCode::BAD_REQUEST,
            "A user with this email already exists".to_string(),
        ));
    }
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    data.people.push(User {
        name: req.name,
        username: req.username.clone(),
        password: hashed_password,
        email: req.email,
    });
    data.total_records += 1;

    match state.persist.save::<UserData>("data", data) {
        Ok(_) => {
            let token = create_jwt(req.username.clone()).await;
            Ok((StatusCode::CREATED, token))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn login(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let data_result = state.persist.load::<UserData>("data");
    let data = match data_result {
        Ok(data) => data,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    // Find the user with the provided email
    let user = data.people.iter().find(|person| person.email == req.email);

    match user {
        Some(user) => {
            // Check if the provided password matches the stored password
            if bcrypt::verify(&req.password, &user.password).is_ok_and(|x| x) {
                let token = create_jwt(user.username.clone()).await;
                Ok((StatusCode::OK, token))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid password.".to_string()))
            }
        }
        None => Err((StatusCode::NOT_FOUND, "User not found.".to_string())),
    }
}
async fn private_route(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = auth_header.token();
    match validate_jwt(token) {
        Ok(_claims) => {
            // If the token is valid, return a success message
            (StatusCode::OK, "You have accessed a private route!")
        }
        Err(_e) => {
            // If the token is not valid, return an error message
            (StatusCode::UNAUTHORIZED, "Invalid token.")
        }
    }
}

async fn create_jwt(username: String) -> String {
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
fn validate_jwt(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret("zen-lang-org".as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn signup_routes(persist: PersistInstance) -> Router {
    let state = Arc::new(MyState {
        persist: Arc::new(persist),
    });

    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/private", get(private_route))
        .layer(AddExtensionLayer::new(state))
}
