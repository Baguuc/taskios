impl crate::repositories::projects::ProjectsRepository {
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &taskios_domain::params::repositories::project::RetrieveParams,
        client: A
    ) -> Result<taskios_domain::Project, sqlx::Error> {
        let mut client = client.acquire().await?; 
        let sql = "SELECT
          p.id, 
          p.name,
          ARRAY_AGG(JSON_BUILD_OBJECT('id', t.id, 'title', t.title, 'description', t.description, 'completion', t.completion)) AS tasks
        FROM 
          projects p
        INNER JOIN 
          tasks t 
          ON 
          t.project_id = p.id
        WHERE
          p.id = $1
        GROUP BY
          p.id, p.name
        ;";

        let raw: RawData = sqlx::query_as(sql)
            .bind(&params.id)
            .fetch_one(&mut *client)
            .await?;
        
        return Ok(taskios_domain::Project::from(raw));
    }
}

#[derive(sqlx::FromRow)]
struct RawData {
    id: i32, 
    name: String,
    tasks: Vec<serde_json::Value>
}

impl From<RawData> for taskios_domain::Project {
    fn from(raw: RawData) -> taskios_domain::Project {
        let tasks = raw.tasks.iter()
            .map(|task| serde_json::from_value(task.clone()).unwrap())
            .collect::<Vec<taskios_domain::Task>>();

        return taskios_domain::Project {
            id: raw.id,
            name: raw.name,
            tasks
        };
    }
}
