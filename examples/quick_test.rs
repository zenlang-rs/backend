#![allow(unused)] // For beginning only

use http::{header::HeaderValue, HeaderMap}; // 0.1.17
use reqwest::{header, Response};
use serde_json::json;
use shuttle_runtime::tokio::{self};
use std::{collections::HashMap, io};
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:8000/api".to_string();

    // Test /health endpoint
    println!("\n============= HEALTH CHECK ==================");
    let res = client.get(url.clone() + "/health").send().await?;
    assert!(res.status().is_success());
    print_response(res).await;

    // Test /compile endpoint
    println!("\n============= COMPILE CHECK =================");
    let res = client
        .post(url.clone() + "/compile")
        .json(&json!({
            "code": "SAMPLE CHECK"
        }))
        .send()
        .await?;
    assert!(res.status().is_success());
    print_response(res).await;

    // Test /signup endpoint
    println!("\n============= SIGNUP CHECK ==================");
    let res = client
        .post(url.clone() + "/signup")
        .json(&json!({
            "name": "Zen-lang-test",
            "username": "zen",
            "password": "lang123",
            "email": "codermohit19@gmail.com"
        }))
        .send()
        .await?;
    assert!(res.status().is_success());
    print_response(res).await;

    // Test /login endpoint
    println!("\n====== LOGIN CHECK WITH WRONG PASSWORD ======");
    let res = client
        .post(url.clone() + "/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"password":"lang","email":"codermohit19@gmail.com"}"#)
        .send()
        .await?;

    assert!(res.status().is_client_error());
    print_response(res).await;

    // Test /login endpoint
    println!("\n======== LOGIN CHECK WITH WRONG EMAIL =======");
    let res = client
        .post(url.clone() + "/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"password":"lang123","email":"codermohit@gmail.com"}"#)
        .send()
        .await?;

    assert!(res.status().is_client_error());
    print_response(res).await;

    // Test /login endpoint
    println!("\n============== LOGIN CHECK  =================");
    let res = client
        .post(url.clone() + "/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"password":"lang123","email":"codermohit19@gmail.com"}"#)
        .send()
        .await?;

    assert!(res.status().is_success());
    print_response(res).await;

    // Test /send_email/:email endpoint
    println!("\n======== SEND EMAIL - FORGET PASSWORD =======");
    let res = client
        .post(url.clone() + "/send_email/codermohit19@gmail.com")
        .send()
        .await?;
    assert!(res.status().is_success());
    print_response(res).await;

    // Test /reset endpoint
    println!("\n============ RESET PASSWORD =================");
    let mut verification_token = String::new();
    io::stdin()
        .read_line(&mut verification_token)
        .expect("failed to readline");
    verification_token.truncate(verification_token.len() - 1);
    let res = client
        .post(url.clone() + "/reset")
        .json(&json!({
            "email": "codermohit19@gmail.com",
            "verification_token": verification_token.to_string(),
            "new_password": "new_pass123"
        }))
        .send()
        .await?;
    print_response(res).await;

    println!("\n====== LOGIN CHECK WITH NEW PASSWORD  =======");
    let res = client
        .post(url.clone() + "/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"password":"new_pass123","email":"codermohit19@gmail.com"}"#)
        .send()
        .await?;
    assert!(res.status().is_success());
    let token = print_response(res).await;

    // Test /changepassword endpoint
    println!("\n============ CHANGE PASSWORD ================");
    let res = client
        .post(url.clone() + "/changepassword")
        .bearer_auth(token)
        .json(&json!({
            "new_password": "new_pass_changed"
        }))
        .send()
        .await?;
    assert!(res.status().is_success());
    print_response(res).await;

    println!("\n====== LOGIN CHECK WITH NEW PASSWORD  =======");
    let res = client
        .post(url.clone() + "/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(r#"{"password":"new_pass_changed","email":"codermohit19@gmail.com"}"#)
        .send()
        .await?;

    assert!(res.status().is_success());
    let token = print_response(res).await;

    Ok(())
}

async fn print_response(res: Response) -> String {
    println!("\n=== Response for : {}", res.url());
    println!("=>  Status: {}", res.status());
    let body = res.text().await.unwrap();
    println!("=>  Body:\n{}\n ", body);
    body
}
