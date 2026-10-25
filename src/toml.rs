use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct Config {
    app: AppConfig,
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct AppConfig {
    name: String,
    debug: bool,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct DatabaseConfig {
    host: String,
    port: u16,
}

pub fn op() {
    let conf_file = "/Users/wangzebin/Downloads/bar/src/config.toml";
    if !Path::new(conf_file).exists() {
        println!("config file does not exist");
        return;
    }

    let content = std::fs::read_to_string(conf_file).unwrap_or_default();
    if content.trim() == "" {
        println!("invalid config file");
        return;
    }
    let config: Config = toml::from_str(&content).unwrap_or_default();

    println!("{:?}", config);
    println!("应用名称: {}", config.app.name);
    println!("服务器端口: {}", config.server.port);
    println!("数据库地址: {}", config.database.host);
}
