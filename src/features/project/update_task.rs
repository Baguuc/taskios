pub struct ProjectUpdateTaskFeature;

impl ProjectUpdateTaskFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectUpdateTaskParams<'p>,
        database_connection: A,
        authios_client: authios_sdk::AuthiosClient
    ) -> Result<(), crate::errors::feature::ProjectUpdateTaskError> {
        use crate::models::Task;
        use crate::errors::feature::ProjectUpdateTaskError as Error;
        use authios_sdk::requests::{
            LoggedUserCheckServicePermissionRequest as ServicePermissionRequest,
            LoggedUserCheckResourcePermissionRequest as ResourcePermissionRequest
        };
        use authios_sdk::responses::{
            LoggedUserCheckServicePermissionResponse as ServicePermissionResponse,
            LoggedUserCheckResourcePermissionResponse as ResourcePermissionResponse
        };
        
        let mut database_connection = database_connection.acquire()
            .await
            .unwrap();

        let service_permission_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .service()
            .check(ServicePermissionRequest {
                service_id: String::from("taskios")
            })
            .await;

        match service_permission_response {
            ServicePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized);
            },
            ServicePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ServicePermissionResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            ServicePermissionResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down"),
            ServicePermissionResponse::PermissionNotFound => panic!("AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software")
        };
        
        let resource_permission_response = authios_client.query()
            .user()
            .logged(params.token.clone())
            .permissions()
            .resource()
            .check(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("project"),
                resource_id: params.task_id.to_string(),
                permission_name: String::from("write")
            })
            .await;

        match resource_permission_response {
            ResourcePermissionResponse::Ok { has_permission } => if !has_permission {
                return Err(Error::Unauthorized);
            },
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            ResourcePermissionResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down"),
            ResourcePermissionResponse::PermissionNotFound => panic!("AUTH SERVER ERROR: auth server wasn't inited - it's lacking crucial permissions to run this software")
        };
        
        let sql = "SELECT title, description, done FROM tasks WHERE id = $1;";
        let task: Task = sqlx::query_as(sql)
            .bind(params.task_id)
            .fetch_one(&mut *database_connection)
            .await
            .map_err(|_| Error::TaskNotFound)?;

        let sql = "UPDATE tasks SET title = $1, description = $2, done = $3 WHERE id = $4;";
        let _ = sqlx::query(sql)
            .bind(params.new_title.unwrap_or(task.title.clone()))
            .bind(params.new_description.unwrap_or(task.description.clone()))
            .bind(params.new_done.unwrap_or(task.done))
            .bind(params.task_id)
            .execute(&mut *database_connection)
            .await;

        Ok(())
    }
}
