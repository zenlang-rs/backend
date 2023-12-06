use crate::controllers::authentication::{MyState, UserData};
use axum::{extract, Json};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use bcrypt::{hash, DEFAULT_COST};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ChangePasswordResponse {
    status_code: u16,
    message: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    new_password: String,
}

pub async fn change_password(
    TypedHeader(auth_header): TypedHeader<Authorization<Bearer>>,
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Json(req): Json<ChangePasswordRequest>,
) -> Json<ChangePasswordResponse> {
    // Verify the JWT and extract the username
    let token = auth_header.token();
    match crate::controllers::authentication::validate_jwt(
        token,
        state
            .secrets
            .get("SECRET_KEY")
            .unwrap_or_else(|| panic!("Expected SECRET_KEY in secrets!")),
    ) {
        Ok(claims) => {
            let username = claims.sub;

            // Load the user data
            let data_result = state.persist.load::<UserData>("data");
            let mut data = match data_result {
                Ok(data) => data,
                Err(e) => {
                    return Json(ChangePasswordResponse {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                        message: e.to_string(),
                    })
                }
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
                    Ok(_) => Json(ChangePasswordResponse {
                        status_code: StatusCode::OK.into(),
                        message: "Password changed successfully".to_string(),
                    }),
                    Err(e) => Json(ChangePasswordResponse {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                        message: e.to_string(),
                    }),
                }
            } else {
                Json(ChangePasswordResponse {
                    status_code: StatusCode::NOT_FOUND.into(),
                    message: "User not found.".to_string(),
                })
            }
        }
        Err(_e) => Json(ChangePasswordResponse {
            status_code: StatusCode::UNAUTHORIZED.into(),
            message: "Invalid token.".to_string(),
        }),
    }
}
