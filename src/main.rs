use std::fs::File;
use std::io;
use std::io::Read;
use chrono::Local;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::config::Config;

mod config;
mod sender;

fn main() {
    let config = read_config();
    println!("{:?}", config);


    let file_appender = tracing_appender::rolling::daily(config.dev.log.path, config.dev.log.prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();
}

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

pub fn read_config() -> Config {
    let mut file = File::open("config/app.toml").expect("config/app.toml does not exist!");
    let mut config_str = String::new();
    file.read_to_string(&mut config_str).expect("Read app.toml fail!");
    toml::from_str(&config_str).unwrap()
}