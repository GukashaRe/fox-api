pub mod server_config;

use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder, Result as ActixResult, post};
use anyhow::Result;
use env_logger::Env;
use serde_json::Value;
use sqlx::{PgPool, Row, query};

#[post("/api/post/test")]
async fn post_json(_data: Json<Value>, base: Data<PgPool>) -> ActixResult<impl Responder> {
    let res = query("SELECT reason,qq FROM group_blacklist WHERE qq = $1")
        .bind(1000)
        .fetch_one(base.as_ref())
        .await
        .map_err(|e| {
            println!("{}:{} 数据库查询失败 {}", file!(), column!(), e);
            ErrorInternalServerError("database err")
        })?;
    let s: String = res.get("reason");
    Ok(HttpResponse::Ok().body(s))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let config = server_config::check_or_crate_config().await?;
    let database = PgPool::connect(&config.pgsql_url).await?;
    Ok(HttpServer::new(move || {
        App::new()
            .service(post_json)
            .app_data(Data::new(database.clone()))
    })
    .bind(("127.0.0.1", 10013))?
    .run()
    .await?)
}
