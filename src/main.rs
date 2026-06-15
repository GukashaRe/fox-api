pub mod server_config;

use actix_web::web::Json;
use actix_web::{App, HttpServer, Result as ActixResult, post};
use anyhow::Result;
use serde_json::Value;

#[post("/post_json")]
async fn post_json(data: Json<Value>) -> ActixResult<String> {
    Ok(data.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = server_config::check_or_crate_config().await?;
    Ok(HttpServer::new(|| App::new().service(post_json))
        .bind(("127.0.0.1", 10013))?
        .run()
        .await?)
}
