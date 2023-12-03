use crate::controllers::authentication::{MyState, User, UserData};
use axum::{extract, Json};
use bcrypt::{hash, DEFAULT_COST};
use http::StatusCode;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    status_code: u16,
    message: String,
    token: Option<String>,
}
pub async fn signup(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<User>,
) -> Result<Json<SignupResponse>, Json<SignupResponse>> {
    if req.email.trim().is_empty() {
        return Ok(Json(SignupResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "Email cannot be empty".to_string(),
            token: None,
        }));
    } else if req.password.trim().is_empty() {
        return Ok(Json(SignupResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "Password cannot be empty".to_string(),
            token: None,
        }));
    } else if req.name.trim().is_empty() {
        return Ok(Json(SignupResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "Name cannot be empty".to_string(),
            token: None,
        }));
    } else if req.username.trim().is_empty() {
        return Ok(Json(SignupResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "UserName cannot be empty".to_string(),
            token: None,
        }));
    }
    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => {
            return Ok(Json(SignupResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                message: e.to_string(),
                token: None,
            }))
        }
    };
    // Check if a user with the same email already exists
    if data.people.iter().any(|person| person.email == req.email) {
        return Ok(Json(SignupResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "A user with this email already exists".to_string(),
            token: None,
        }));
    }
    let hashed_password = hash(&req.password, DEFAULT_COST).unwrap();
    data.people.push(User {
        name: req.name,
        username: req.username.clone(),
        password: hashed_password,
        email: req.email,
        verification_code: None,
    });
    data.total_records += 1;

    match state.persist.save::<UserData>("data", data) {
        Ok(_) => {
            let token = crate::controllers::authentication::create_jwt(req.username.clone()).await;
            // Return a JSON response with status code and message
            let response = SignupResponse {
                status_code: StatusCode::CREATED.into(),
                message: "User created successfully".to_string(),
                token: Some(token),
            };
            Ok(Json(response))
        }
        Err(e) => Ok(Json(SignupResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: e.to_string(),
            token: None,
        })),
    }
}
