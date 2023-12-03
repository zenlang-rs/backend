use std::sync::Arc;
use axum::{extract, Json};
use axum::extract::Path;
use dotenv::dotenv;
use http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::controllers::auth::email_util::Email;
use crate::controllers::authentication::{MyState, User, UserData};
use crate::smtp_config;

#[derive(Deserialize)]
pub struct EmailParam {
    email: String,
}

#[derive(Debug, Serialize)]
pub struct SendEmailResponse {
    status_code: u16,
    message: String,
}
pub async fn send_email(
    extract::Extension(state): extract::Extension<Arc<MyState>>,
    Path(EmailParam { email }): Path<EmailParam>,
) -> Json<SendEmailResponse> {
    dotenv().ok();
    let config = smtp_config::Config::init(email.clone());

    // Generate a random verification code
    let verification_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(100)
        .map(char::from)
        .collect();

    let verification_url = format!(
        "{}/{}/{}",
        config.reset_password_url, email, verification_code
    );

    let data_result = state.persist.load::<UserData>("data");
    let mut data = match data_result {
        Ok(data) => data,
        Err(e) => return Json(SendEmailResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: e.to_string(),
        }),
    };

    // Initialize a mutable variable for the user
    let mut user_option: Option<&mut User> = None;

    // Find the user and update the verification code
    for user in data.people.iter_mut() {
        if user.email == email {
            user.verification_code = Some(verification_code.clone());
            user_option = Some(user);
            break;
        }
    }

    // Check if a user was found
    let user = match user_option {
        Some(user) => user,
        None => {
            return Json(SendEmailResponse {
                status_code: StatusCode::BAD_REQUEST.into(),
                message: "A user with this email does not exist".to_string(),
            })
        }
    };

    //  Create an Email instance
    let email = Email::new(user.clone(), verification_url, config);
    // Send a password reset token email
    if let Err(err) = email.send_reset_password_code().await {
        eprintln!("Failed to send password reset token email: {:?}", err);
        Json(SendEmailResponse {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            message: format!("Failed to send email: {:?}", err),
        })
    } else {
        // println!("✅Password reset token email sent successfully!");
        match state.persist.save::<UserData>("data", data) {
            Ok(_) => Json(SendEmailResponse {
                status_code: StatusCode::OK.into(),
                message: "Email sent successfully!".to_string(),
            }),
            Err(e) => Json(SendEmailResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.into(),
                message: e.to_string(),
            }),
        }
    }
}