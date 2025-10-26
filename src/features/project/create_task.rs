pub struct ProjectCreateTaskFeature;

impl ProjectCreateTaskFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectCreateTaskParams<'p>,
        database_connection: A,
        authios_client: authios_sdk::AuthiosClient
    ) -> Result<(), crate::errors::feature::ProjectCreateTaskError> {
        use crate::errors::feature::ProjectCreateTaskError as Error;
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
                resource_id: params.project_id.to_string(),
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

        let sql = "INSERT INTO tasks (name, description, done, project_id) VALUES ($1, $2, false, $3);";
        let result = sqlx::query(sql)
            .bind(params.project_id)
            .execute(&mut *database_connection)
            .await;

        match result {
            Err(error) => match error {
                sqlx::Error::Database(error) if error.is_foreign_key_violation() => Err(Error::ProjectNotFound),
                _ => panic!("something went wrong: an unexpected error has happened.")
            },
            _ => Ok(())
        }
    }
}
