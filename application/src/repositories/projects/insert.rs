impl crate::repositories::projects::ProjectsRepository {
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::repositories::project::InsertParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let mut client = client.acquire().await?; 
        let sql = "INSERT INTO projects (name) VALUES ($1);"; 

        return sqlx::query(sql)
            .bind(&params.name)
            .execute(&mut *client)
            .await;
    }
}
