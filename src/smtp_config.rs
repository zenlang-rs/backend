use std::sync::Arc;

use shuttle_runtime::SecretStore;

#[derive(Debug, Clone)]
pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
    pub smtp_to: String,
    pub reset_password_url: String,
}

impl Config {
    pub fn init(smtp_to: String, secrets: Arc<SecretStore>) -> Config {
        let smtp_host = secrets.get("SMTP_HOST").expect("SMTP_HOST must be set");
        let smtp_port = secrets.get("SMTP_PORT").expect("SMTP_PORT must be set");
        let smtp_user = secrets.get("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = secrets.get("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_from = secrets.get("SMTP_FROM").expect("SMTP_FROM must be set");
        let reset_password_url = secrets
            .get("RESET_PASSWORD_URL")
            .expect("RESET_PASSWORD_URL must be set");
        // let smtp_to = secrets.get("SMTP_TO").expect("SMTP_TO must be set");

        Config {
            smtp_host,
            smtp_pass,
            smtp_user,
            smtp_port: smtp_port.parse::<u16>().unwrap(),
            smtp_from,
            smtp_to,
            reset_password_url,
        }
    }
}
