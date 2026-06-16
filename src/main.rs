pub mod fox_logger;
pub mod server_config;

// 重导出关键的函数，这样 $crate::get_logger() 就能找到了
use crate::fox_logger::{LogLevel, Logger, init_logger};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, Json};
use actix_web::{App, HttpResponse, HttpServer, Responder, Result as ActixResult, get, post};
use anyhow::Result;
use env_logger::Env;
pub use fox_logger::get_logger;
pub use fox_logger::init_default;
pub use fox_logger::init_with_file;
use serde_json::Value;
use sqlx::{PgPool, Row, query};

#[post("/api/post/test")]
async fn post_json(_data: Json<Value>, base: Data<PgPool>) -> ActixResult<impl Responder> {
    let res = query("SELECT reason,qq FROM group_blacklist WHERE qq = $1")
        .bind(1000)
        .fetch_one(base.as_ref())
        .await
        .map_err(|e| {
            log_error!("数据库查询失败 {}", e);
            ErrorInternalServerError("database err")
        })?;
    let s: String = res.get("reason");
    Ok(HttpResponse::Ok().body(s))
}

#[get("/get")]
async fn gget() -> ActixResult<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut logger = Logger::new();
    logger.set_min_level(LogLevel::Debug);
    init_logger(logger).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let config = server_config::check_or_crate_config().await?;
    let database = PgPool::connect(&config.pgsql_url).await?;
    Ok(HttpServer::new(move || {
        App::new()
            .service(post_json)
            .service(gget)
            .app_data(Data::new(database.clone()))
    })
    .bind(("127.0.0.1", 10013))?
    .run()
    .await?)
}
