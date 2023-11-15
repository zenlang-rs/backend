use std::{
    env,
    fmt::Display,
    sync::Arc,
    time::{Duration, SystemTime},
};

use dotenv::dotenv;
use email::Email;
use rand::{distributions::Alphanumeric, Rng};

use crate::{config, email, login_signup::headers::authorization::Bearer};
use axum::{
    extract::{self, Path, TypedHeader},
    headers,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use bcrypt::{hash, DEFAULT_COST};
use headers::Authorization;
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tower_http::add_extension::AddExtensionLayer;
#[derive(Deserialize, Serialize, Debug)]
pub struct UserData {
    people: Vec<User>,
    total_records: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub name: String,
    username: String,
    password: String,
    pub email: String,
    verification_token: Option<String>,
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

pub struct MyState {
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
    once_cell::sync::Lazy::new(|| {
        let secret = env::var("SECRET_KEY").unwrap_or_else(|_| panic!("SECRET_KEY must be set"));
        jsonwebtoken::EncodingKey::from_secret(secret.as_ref())
    });

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
        verification_token: None,
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

pub async fn login(
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

#[derive(Deserialize)]
struct EmailParam {
    email: String,
}

async fn send_email(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Path(EmailParam { email }): Path<EmailParam>,
) -> impl IntoResponse {
    dotenv().ok();
    let config = config::Config::init(email.clone());

    // Generate a random verification code
    let verification_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(100)
        .map(char::from)
        .collect();

    let verification_url = format!(
        "{}/{}/{}",
        config.reset_password_url, email, verification_token
    );

    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    // Initialize a mutable variable for the user
    let mut user_option: Option<&mut User> = None;

    // Find the user and update the verification code
    for user in data.people.iter_mut() {
        if user.email == email {
            user.verification_token = Some(verification_token.clone());
            user_option = Some(user);
            break;
        }
    }

    // Check if a user was found
    let user = match user_option {
        Some(user) => user,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "A user with this email does not exist".to_string(),
            )
        }
    };

    //  Create an Email instance
    let email = Email::new(user.clone(), verification_url, config);
    // Send a password reset token email
    if let Err(err) = email.send_reset_password_code().await {
        eprintln!("Failed to send password reset token email: {:?}", err);
    } else {
        // println!("✅Password reset token email sent successfully!");
        match state.persist.save::<UserData>("data", data) {
            Ok(_) => (StatusCode::OK, "Data saved successfully".to_string()),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
    }
    (StatusCode::OK, "Emails sent successfully".to_string())
}

#[derive(Deserialize)]
struct ResetPasswordParam {
    email: String,
    verification_token: String,
    new_password: String,
}

async fn reset_password(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(ResetPasswordParam {
        email,
        verification_token,
        new_password,
    }): Json<ResetPasswordParam>,
) -> impl IntoResponse {
    // Load the user data from the state
    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    let hashed_password = hash(new_password, DEFAULT_COST).unwrap();
    // Find the user with the same email and verification token
    if let Some(user) = data.people.iter_mut().find(|person| {
        person.email == email && person.verification_token.as_deref() == Some(&verification_token)
    }) {
        // Update the user's password
        user.password = hashed_password;

        match state.persist.save::<UserData>("data", data) {
            Ok(_) => (StatusCode::OK, "Data saved successfully".to_string()),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
    } else {
        return (
            StatusCode::BAD_REQUEST,
            "A user with this email and verification token does not exist".to_string(),
        );
    }

    (StatusCode::OK, "Password reset successfully".to_string())
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    new_password: String,
}

async fn change_password(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Verify the JWT and extract the username
    let token = auth_header.token();
    match validate_jwt(token) {
        Ok(claims) => {
            let username = claims.sub;

            // Load the user data
            let data_result = state.persist.load::<UserData>("data");
            let mut data = match data_result {
                Ok(data) => data,
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            };
            // Find the user and update their password
            if let Some(user) = data
                .people
                .iter_mut()
                .find(|person| person.username == username)
            {
                let hashed_password = hash(req.new_password, DEFAULT_COST).unwrap();
                user.password = hashed_password;

                match state.persist.save::<UserData>("data", data) {
                    Ok(_) => Ok((StatusCode::OK, "Password changed successfully".to_string())),
                    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
                }
            } else {
                Err((StatusCode::NOT_FOUND, "User not found.".to_string()))
            }
        }
        Err(_e) => Err((StatusCode::UNAUTHORIZED, "Invalid token.".to_string())),
    }
}
