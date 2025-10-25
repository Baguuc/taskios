pub struct ProjectCreateFeature;

impl ProjectCreateFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectCreateParams<'p>,
        database_connection: A
    ) {
        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();
        
        let sql = "INSERT INTO projects (name) VALUES ($1);";
        let _ = sqlx::query(sql)
            .bind(params.name)
            .execute(&mut *database_connection)
            .await;
    }
}
