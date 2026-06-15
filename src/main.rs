pub mod server_config;

use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder, Result as ActixResult, post};
use anyhow::Result;
use serde_json::Value;
use sqlx::{PgPool, Row, query};

#[post("/post_json")]
async fn post_json(_data: Json<Value>, base: Data<PgPool>) -> ActixResult<impl Responder> {
    let res = query("SELECT reason FROM group_blacklist gb WHERE qq = $1")
        .bind(1000)
        .fetch_one(base.as_ref())
        .await
        .map_err(|e| {
            eprintln!("数据库查询失败 {}", e);
            ErrorInternalServerError("database err")
        })?;
    let s: String = res.get("reason");
    Ok(HttpResponse::Ok().body(s))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = server_config::check_or_crate_config().await?;
    let database = PgPool::connect(&config.pgsql_url).await?;
    Ok(
        HttpServer::new(move || App::new().service(post_json).app_data(database.clone()))
            .bind(("127.0.0.1", 10013))?
            .run()
            .await?,
    )
}
