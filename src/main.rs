use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
mod config;
mod email;
pub mod login_signup;
use dotenv::dotenv;
// use http::{Method, header::ACCESS_CONTROL_ALLOW_ORIGIN};
use login_signup::{auth_routes, UserData};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;
use tower_http::cors::CorsLayer;
use zen::{compile, get_version};

async fn hello_zen() -> &'static str {
    format!(
        "Zen is High Dear!\nCompiler Version: {version}",
        version = get_version().unwrap_or("v0.0".to_owned())
    )
    .leak()
}

async fn compile_code(
    extract::Json(user): extract::Json<CodeCompileRequest>,
) -> Json<CodeOutputResponse> {
    Json(CodeOutputResponse {
        output: compile(user.code),
    })
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
