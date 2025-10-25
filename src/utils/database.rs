const DATABASE_STRUCTURE: &str = "
CREATE DATABASE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE DATABASE tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    state_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL

    FOREIGN KEY state_id REFERENCES task_states(id),
    FOREIGN KEY project_id REFERENCES projects(id)
);

CREATE DATABASE task_states (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO task_states (id, name) VALUES:q
;
";

/// # migrate
///
/// Migrate the database to the state necessary for the program to function properly.
///
/// ### Arguments:
/// + client: [sqlx::postgres::PgPool]
///
pub async fn migrate(client: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    use sqlx::query;
    use clin::components::{progress_bar, error};

    let mut tx = client.begin().await?;

    let count = MIGRATIONS.len();
    let mut curr_count = 0;

    progress_bar(count, curr_count);
    
    for sql in MIGRATIONS {
        curr_count += 1;

        match query(sql).execute(&mut *tx).await {
            Ok(_) => {},
            Err(_) => {
                error("Migration command failed.", format!("SQL:\n{}", sql));
                
                std::process::exit(1);
            }
        }; 

        progress_bar(count, curr_count);
    }

    let _ = tx.commit().await?;
    
    return Ok(());
}

/// # create_pool
///
/// creates a database pool from database credentials wrapped in [crate::config::DatabaseConfig]
/// struct.
pub async fn create_pool(config: crate::config::DatabaseConfig) -> Result<sqlx::postgres::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPool;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );
    let pool = PgPool::connect(connection_string.as_str()).await?;

    return Ok(pool);
}
