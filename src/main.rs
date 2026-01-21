mod models;
mod features;
mod params;
mod errors;
mod utils;
mod config;
mod extractors;
mod repositories;
mod cli;
mod web;

#[actix_web::main]
async fn main() {
    cli::MainCli::run().await;
}

