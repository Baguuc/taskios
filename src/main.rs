mod models;
mod features;
mod params;
mod errors;
mod utils;
mod config;
mod extractors;
mod repositories;

#[actix_web::main]
async fn main() {
    run_api(config::Config::read("config.json".to_string()).unwrap()).await.unwrap();
}

pub async fn run_api(config: crate::config::Config) -> Result<(), crate::errors::web::ServerRunError> {
    use actix_web::web::{
        PathConfig,
        QueryConfig,
        JsonConfig
    };
    use crate::utils::database::create_pool;
    use crate::errors::web::{
        PathDeserializeError,
        QueryDeserializeError,
        JsonDeserializeError,
        ServerRunError as Error
    };

    let port = config.port;
    let database_client = create_pool(config.database.clone()).await?;
    let authios_client = authios_sdk::AuthiosClient::new(config.auth.url.clone())
        .ok_or(Error::AuthiosConnection)?;

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            // customize errors
            .app_data(PathConfig::default().error_handler(|err, _req| PathDeserializeError(err).into()))
            .app_data(QueryConfig::default().error_handler(|err, _req| QueryDeserializeError(err).into()))
            .app_data(JsonConfig::default().error_handler(|err, _req| JsonDeserializeError(err).into()))
            // add shared data 
            .app_data(actix_web::web::Data::new(database_client.clone()))
            .app_data(actix_web::web::Data::new(authios_client.clone()))
            .app_data(actix_web::web::Data::new(config.clone()))
            // register features 
            .configure(features::ProjectCreateFeature::register)
            .configure(features::ProjectListFeature::register)
            .configure(features::ProjectDeleteFeature::register)
            .configure(features::ProjectUpdateFeature::register)
            .configure(features::TaskCreateFeature::register)
            .configure(features::TaskListFeature::register)
            .configure(features::TaskUpdateFeature::register)
            .configure(features::TaskDeleteFeature::register)
    });
    server.bind(("0.0.0.0", port))?
        .run()
        .await?;

    return Ok(());
}
