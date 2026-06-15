use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::NotFound;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub pgsql_url: String,
}

impl Config {
    fn example() -> Self {
        Self {
            pgsql_url: "postgresql://user:password@localhost:5432/dbname".to_string(),
        }
    }
}

pub async fn check_or_crate_config() -> Result<Config> {
    let config_path = "config.toml";

    match File::open(config_path).await {
        // 文件存在，读取并解析
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .await
                .context("Failed to read config file")?;

            let config: Config =
                toml::from_str(&contents).context("Failed to parse config file as TOML")?;

            Ok(config)
        }
        // 文件不存在，创建默认配置并写入
        Err(e) if e.kind() == NotFound => {
            let default_config = Config::example();
            let toml_string =
                toml::to_string(&default_config).context("Failed to serialize default config")?;

            // 写入文件
            tokio::fs::write(config_path, toml_string)
                .await
                .context("Failed to write default config file")?;

            Ok(default_config)
        }
        // 其他错误，向上传播
        Err(e) => {
            Err(anyhow!(e)).context(format!("Failed to open config file at '{}'", config_path))
        }
    }
}
