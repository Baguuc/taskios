impl crate::repositories::tasks::TasksRepository {
    pub async fn insert<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::repositories::task::InsertParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let mut client = client.acquire().await?; 
        let sql = "INSERT INTO tasks (title, description, project_id) VALUES ($1, $2, $3);"; 

        return sqlx::query(sql)
            .bind(&params.title)
            .bind(&params.description)
            .bind(&params.project_id)
            .execute(&mut *client)
            .await;
    }
}
