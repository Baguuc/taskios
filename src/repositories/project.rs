pub struct ProjectRepository;

impl ProjectRepository {
    /// insert a project into the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project: project's data _without id value_
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        project: &crate::models::ProjectWithoutId,
    ) -> Result<crate::models::Project, crate::errors::repository::ProjectCreateError> {
        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "INSERT INTO projects (name) VALUES ($1) RETURNING id, name;";
        let project = sqlx::query_as(sql)
            .bind(&project.name)
            .fetch_one(&mut *database_connection)
            .await
            // error won't happen
            .unwrap();

        Ok(project)
    }

    /// retrieve project data from the database by the id
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project_id: project's id
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        project_id: &i32,
    ) -> Option<crate::models::Project> {
        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "SELECT id, name FROM projects WHERE id = $1;";
        let result = sqlx::query_as(sql)
            .bind(project_id)
            .fetch_one(&mut *database_connection)
            .await;

        result.ok()
    }

    /// update a project in the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project_id: id of the project to update
    /// * new_data: new data of the project in the form of [crate::models::PartialProject] where None fields mean to not update them
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        project_id: &i32,
        new_data: &crate::models::PartialProject,
    ) -> Result<crate::models::Project, crate::errors::repository::ProjectUpdateError> {
        use crate::errors::repository::ProjectUpdateError as Error;
        use crate::models::ProjectWithoutId;

        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "SELECT name FROM tasks WHERE id = $1;";
        let project: ProjectWithoutId = sqlx::query_as(sql)
            .bind(project_id)
            .fetch_one(&mut *database_connection)
            .await
            .map_err(|_| Error::ProjectNotFound)?;

        let sql = "UPDATE projects SET name = $2 WHERE id = $1 RETURNING id, name;";
        let result = sqlx::query_as(sql)
            .bind(project_id)
            .bind(new_data.clone().name.unwrap_or(project.name))
            .fetch_one(&mut *database_connection)
            .await;

        result.map_err(|_| Error::ProjectNotFound)
    }

    /// delete a project from the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project_id: id of the project to delete
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        project_id: &i32,
    ) -> Result<(), crate::errors::repository::ProjectDeleteError> {
        use crate::errors::repository::ProjectDeleteError as Error;

        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "DELETE FROM projects WHERE id = $1;";
        let result = sqlx::query(sql)
            .bind(project_id)
            .execute(&mut *database_connection)
            .await
            .unwrap();

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(Error::ProjectNotFound)
        }
    }
}
