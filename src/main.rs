use axum::{routing::{get, post}, Router, Json, extract};
use serde::{Deserialize, Serialize};
use zen::compile;

async fn hello_zen() -> &'static str {
    "Hello, world!"
}

async fn compile_code(extract::Json(user): extract::Json<CodeCompileRequest>) -> Json<CodeOutputResponse> {
    Json(CodeOutputResponse { output: compile(user.code) })
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {

    let router = Router::new()
                            .route("/api/health", get(hello_zen))
                            .route("/api/compile", post(compile_code));

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