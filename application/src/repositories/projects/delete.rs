impl crate::repositories::projects::ProjectsRepository {
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::repositories::project::DeleteParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let mut client = client.acquire().await?; 
        let sql = "DELETE FROM projects WHERE id = $1;"; 

        return sqlx::query(sql)
            .bind(&params.id)
            .execute(&mut *client)
            .await;
    }
}
