use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dev: Dev
}

#[derive(Debug, Deserialize)]
pub struct Dev {
    pub mail: Mail,
    pub log: Log
}

#[derive(Debug, Deserialize)]
pub struct Mail {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub to: Vec<String>,
    pub from: String
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub path: String,
    pub prefix: String
}