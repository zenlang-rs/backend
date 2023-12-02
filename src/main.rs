use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
// use http::{Method, header::ACCESS_CONTROL_ALLOW_ORIGIN};
use login_signup::{auth_routes, UserData};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tower_http::cors::CorsLayer;
use zen::run_program;

mod config;
mod email;

mod login_signup;

async fn hello_zen() -> &'static str {
    format!(
        "Zen is High Dear!\nCompiler Version: v0.2.0")
    .leak()
}

async fn compile_code(
    extract::Json(user): extract::Json<CodeCompileRequest>,
) -> Json<CodeOutputResponse> {
    Json(CodeOutputResponse {
        output: runnable_code(user.code, ""),
    })
}

fn runnable_code(code: String, input: &str) -> Result<String, String> {
    let runnable = run_program(code, input);
    match runnable {
        Ok(output) => Ok(output),
        Err(err) => Err(format!("[ERROR]\n{}\n{}", err.msg, err.error_type)),
    }
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_persist::Persist] persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    // let origins: [axum::http::HeaderValue; 3] = [
    //     "http://localhost:8000".parse().unwrap(),
    //     "http://zenlang.netlify.app".parse().unwrap(),
    //     "https://zenlang.netlify.app".parse().unwrap(),
    // ];
    dotenv().ok();
    if persist.load::<UserData>("data").is_err() {
        persist.save::<UserData>("data", UserData::new()).unwrap();
    }
    let cors = CorsLayer::permissive();

    let api_router = Router::new()
        .route("/health", get(hello_zen))
        .route("/compile", post(compile_code))
        .merge(auth_routes(persist));

    let router = Router::new().nest("/api", api_router).layer(cors);

    Ok(router.into())
}

#[derive(Deserialize)]
struct CodeCompileRequest {
    pub code: String,
}

#[derive(Serialize)]
struct CodeOutputResponse {
    pub output: Result<String, String>,
}
