pub struct ProjectUpdateFeature;

impl ProjectUpdateFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectUpdateParams<'p>,
        database_connection: A
    ) -> Option<()> {
        if params.new_name.is_none() {
            return Some(())
        }

        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();
        
        let sql = "UPDATE projects SET name = $1 WHERE id = $2;";
        let result = sqlx::query(sql)
            .bind(params.new_name.clone().unwrap())
            .execute(&mut *database_connection)
            .await
            .unwrap();

        if result.rows_affected() > 0 { 
            Some(()) 
        } else { 
            None
        }
    }
}
