#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;
use shuttle_runtime::tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8000/api")?;
    hc.do_get("/health").await?.print().await?;
    hc.do_post(
        "/signup",
        json!({"name":"df","username":"zen","password":"lang","email":"cpass@gmail.com"}
        ),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/login",
        json!({"password":"lang","email":"cpass@gmail.com"}
        ),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/login",
        json!({"password":"lang2","email":"cpass@gmail.com"}
        ),
    )
    .await?
    .print()
    .await?;

    // hc.do_get("/private?").await?.print().await?;
    Ok(())
}
