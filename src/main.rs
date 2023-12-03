use axum::{
    routing::{get, post},
    Router,
};
use controllers::authentication::{auth_routes, UserData};
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use tower_http::cors::CorsLayer;

mod controllers;
mod smtp_config;

async fn api_health() -> &'static str {
    "Zen is High Dear!\nCompiler Version: v0.2.0"
        .to_string()
        .leak()
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // let origins: [axum::http::HeaderValue; 3] = [
    //     "http://localhost:8000".parse().unwrap(),
    //     "http://zenlang.netlify.app".parse().unwrap(),
    //     "https://zenlang.netlify.app".parse().unwrap(),
    // ];
    if persist.load::<UserData>("data").is_err() {
        persist.save::<UserData>("data", UserData::new()).unwrap();
    }
    let cors = CorsLayer::permissive();

    let api_router = Router::new()
        .route("/health", get(api_health))
        .route("/compile", post(controllers::compile_code::compile_code))
        .route("/quiz", post(controllers::compile_code::take_quiz))
        .merge(auth_routes(persist, secret_store))
        .layer(cors.clone());

    let router = Router::new().nest("/api", api_router).layer(cors);

    Ok(router.into())
}
