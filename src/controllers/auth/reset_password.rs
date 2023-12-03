use std::sync::Arc;
use axum::{extract, Json};
use bcrypt::{DEFAULT_COST, hash};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::controllers::authentication::{MyState, UserData};

#[derive(Deserialize)]
pub struct ResetPasswordParam {
    email: String,
    verification_token: String,
    new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    status_code: u16,
    message: String,
}

pub async fn reset_password(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(ResetPasswordParam {
             email,
             verification_token,
             new_password,
         }): Json<ResetPasswordParam>,
) -> Json<ResetPasswordResponse> {
    // Load the user data from the state
    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => return Json(ResetPasswordResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: e.to_string(),
        }),
    };
    let hashed_password = hash(new_password, DEFAULT_COST).unwrap();
    // Find the user with the same email and verification token
    if let Some(user) = data.people.iter_mut().find(|person| {
        person.email == email && person.verification_code.as_deref() == Some(&verification_token)
    }) {
        // Update the user's password
        user.password = hashed_password;

        match state.persist.save::<UserData>("data", data) {
            Ok(_) => Json(ResetPasswordResponse {
                status_code: StatusCode::OK.into(),
                message: "Password Reset Successfully!".to_string(),
            }),
            Err(e) => Json(ResetPasswordResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                message: e.to_string(),
            }),
        }
    } else {
        Json(ResetPasswordResponse {
            status_code: StatusCode::BAD_REQUEST.into(),
            message: "A user with this email and verification token does not exist".to_string(),
        })
    }
}