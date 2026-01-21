/// sqlx's migrator object.
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

/// Command for running migrations.
pub async fn command(args: crate::cli::CliFlags) {
    use crate::utils::database::create_pool;
    use crate::utils::error::error_if_necessary;
    use crate::config::Config;
    use clin::components::{
        header,
        success,
        error
    };

    header("Parsing the config");
    let config = error_if_necessary(Config::read(
        args.clone()
            .config
            .unwrap_or(String::from("./taskios.json")),
    ));
    success("Parsed the config");

    header("Connecting to the database");
    let database_client = error_if_necessary(create_pool(config.database.clone()).await);
    let mut database_connection = error_if_necessary(database_client.acquire().await);
    success("Connected to the database");

    header("Running the migrations");
    let result = MIGRATOR.run(&mut *database_connection).await;
    match result {
        Ok(_) => success("Successfully migrated the database!"),
        Err(err) => error("Cannot ran the migrations", err)
    };
}