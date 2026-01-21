pub struct TaskRepository;

impl TaskRepository {
    /// insert a task belonging to a project into the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project_id: id of the project the task is tied to
    /// * task: data of the task to insert _without the id field_
    pub async fn create<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        task: &crate::models::TaskWithoutId,
    ) -> Result<crate::models::Task, crate::errors::repository::TaskCreateError> {
        use crate::errors::repository::TaskCreateError as Error;

        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "INSERT INTO tasks (title, description, done, project_id) VALUES ($1, $2, $3, $4) RETURNING id, title, description, done;";
        let result = sqlx::query_as(sql)
            .bind(&task.title)
            .bind(&task.description)
            .bind(task.done)
            .bind(task.project_id)
            .fetch_one(&mut *database_connection)
            .await;

        result.map_err(|_| Error::ProjectNotFound)
    }

    /// retrieve a task from a database by the id
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * task_id: id of the task to retrieve
    pub async fn retrieve<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        task_id: &i32,
    ) -> Option<crate::models::Task> {
        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "SELECT id, title, description, done, project_id FROM tasks WHERE id = $1;";
        let result = sqlx::query_as(sql)
            .bind(task_id)
            .fetch_one(&mut *database_connection)
            .await;

        result.ok()
    }

    /// list tasks belonging to a project with specified id
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * project_id: project's id
    pub async fn list<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        project_id: &i32,
    ) -> Vec<crate::models::Task> {
        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "SELECT id, title, description, done FROM tasks WHERE project_id = $1;";
        let result = sqlx::query_as(sql)
            .bind(project_id)
            .fetch_all(&mut *database_connection)
            .await;

        result.unwrap_or(vec![])
    }

    /// update a task in the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * task_id: id of the task to update
    /// * new_data: new data of the task in the form of [crate::models::PartialTask] where None field mean to not update them
    pub async fn update<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        task_id: &i32,
        new_data: &crate::models::PartialTask,
    ) -> Result<crate::models::Task, crate::errors::repository::TaskUpdateError> {
        use crate::errors::repository::TaskUpdateError as Error;
        use crate::models::TaskWithoutId;

        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "SELECT title, description, done FROM tasks WHERE id = $1;";
        let task: TaskWithoutId = sqlx::query_as(sql)
            .bind(task_id)
            .fetch_one(&mut *database_connection)
            .await
            .map_err(|_| Error::TaskNotFound)?;

        let sql = "UPDATE tasks SET title = $2, description = $3, done = $4 WHERE id = $1 RETURNING id, title, description, done;";
        let result = sqlx::query_as(sql)
            .bind(task_id)
            .bind(new_data.title.clone().unwrap_or(task.title))
            .bind(new_data.description.clone().unwrap_or(task.description))
            .bind(new_data.done.unwrap_or(task.done))
            .fetch_one(&mut *database_connection)
            .await;

        result.map_err(|_| Error::TaskNotFound)
    }

    /// delete a task from the database
    ///
    /// arguments:
    /// * database_connection: the database connection in the form of an acquirable connection
    /// * task_id: id of the task to delete
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        database_connection: A,
        task_id: &i32,
    ) -> Result<(), crate::errors::repository::TaskDeleteError> {
        use crate::errors::repository::TaskDeleteError as Error;

        let mut database_connection = database_connection.acquire().await.unwrap();

        let sql = "DELETE FROM tasks WHERE task_id = $1;";
        let result = sqlx::query(sql)
            .bind(task_id)
            .execute(&mut *database_connection)
            .await
            .unwrap();

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(Error::TaskNotFound)
        }
    }
}
