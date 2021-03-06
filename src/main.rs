#![feature(proc_macro_hygiene, decl_macro)]

use crate::config::*;
use std::env;

mod bot;
mod config;
mod utils;
mod web;

#[macro_use]
extern crate rocket;

#[tokio::main]
async fn main() {
    let cfg = Config::load("config.toml").unwrap();

    let tracing = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    let ansi = env::var("RUST_LOG_DISABLE_ANSI").unwrap_or("".to_string()) != "1".to_string();
    tracing.with_ansi(ansi).init();

    /*let a = ["ROCKET_PROFILE", "ROCKET_PORT", "ROCKET_ADDRESS"]
    .iter()
    .map(|key| format!("{}={}", key, env::var(key).unwrap_or_default()))
    .collect::<Vec<String>>()
    .join("; ");*/

    // info!("env variables: {}", a);

    info!("config time set to {}:{}", cfg.time_h, cfg.time_m);

    let mut c = bot::new(&cfg).await.expect("failed to start client");

    let cache_http = c.cache_and_http.clone();

    tokio::spawn(async move {
        web::run(&cfg, Box::new(cache_http))
            .await
            .expect("failed to start web app");
    });

    c.start().await.expect("failed to start bot");
}
