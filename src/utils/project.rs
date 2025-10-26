pub async fn project_exists<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(id: &i32, database_connection: A) -> bool {
    let mut database_connection = database_connection.acquire()
        .await
        .unwrap();

    let sql = "SELECT p.id FROM projects p WHERE p.id = $1;";
    let result = sqlx::query(sql)
        .bind(id)
        .execute(&mut *database_connection)
        .await
        .unwrap();

    result.rows_affected() > 0
}
