use std::sync::Arc;
use axum::{extract, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::controllers::authentication::{create_jwt, MyState, UserData};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    pub(crate) password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    status_code: u16,
    message: String,
    token: Option<String>,
}
pub async fn login(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Json<LoginResponse>> {
    let data_result = state.persist.load::<UserData>("data");
    let data = match data_result {
        Ok(data) => data,
        Err(e) => return Ok(Json(LoginResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: e.to_string(),
            token: None,
        })),
    };

    // Find the user with the provided email
    let user = data.people.iter().find(|person| person.email == req.email);

    match user {
        Some(user) => {
            // Check if the provided password matches the stored password
            if bcrypt::verify(&req.password, &user.password).is_ok_and(|x| x) {
                let token = create_jwt(user.username.clone()).await;
                Ok(Json(LoginResponse {
                    status_code: StatusCode::OK.into(),
                    message: "Login successful".to_string(),
                    token: Some(token),
                }))
            } else {
                Ok(Json(LoginResponse {
                    status_code: StatusCode::UNAUTHORIZED.into(),
                    message: "Invalid password.".to_string(),
                    token: None,
                }))
            }
        }
        None => Ok(Json(LoginResponse {
            status_code: StatusCode::NOT_FOUND.into(),
            message: "User not found.".to_string(),
            token: None,
        })),
    }
}