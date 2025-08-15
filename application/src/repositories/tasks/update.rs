impl crate::repositories::tasks::TasksRepository {
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::repositories::task::UpdateParams,
        client: A
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let mut client = client.acquire().await?;

        let sql = "SELECT title, description, completion FROM tasks WHERE id = $1";
        let task: RawData = sqlx::query_as(sql)
            .bind(&params.id)
            .fetch_one(&mut *client)
            .await?;

        let title = params.new.title.clone().unwrap_or(task.title);
        let description = params.new.description.clone().unwrap_or(task.description);
        let completion = params.new.completion.clone().unwrap_or(task.completion);

        let sql = "UPDATE tasks SET title = $2, description = $3, completion = $4 WHERE id = $1;"; 

        return sqlx::query(sql)
            .bind(&params.id)
            .bind(&title)
            .bind(&description)
            .bind(&completion)
            .execute(&mut *client)
            .await;
    }
}

#[derive(sqlx::FromRow)]
pub struct RawData {
    title: String,
    description: String,
    completion: String
}
