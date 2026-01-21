mod cli;
mod config;
mod errors;
mod extractors;
mod features;
mod models;
mod params;
mod repositories;
mod utils;
mod web;

#[actix_web::main]
async fn main() {
    cli::MainCli::run().await;
}
