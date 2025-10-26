pub struct ProjectListFeature;

impl ProjectListFeature {
    pub async fn execute<'p, A: sqlx::Acquire<'p, Database = sqlx::Postgres>>(
        params: crate::params::feature::ProjectListParams<'p>,
        database_connection: A,
        authios_client: authios_sdk::AuthiosClient
    ) -> Result<Option<Vec<crate::models::UserProject>>, crate::errors::feature::ProjectListError> {
        use crate::models::UserProject; 
        use crate::errors::feature::ProjectListError as Error;
        use authios_sdk::requests::{
            LoggedUserCheckServicePermissionRequest as ServicePermissionRequest,
            LoggedUserListResourcePermissionsRequest as ResourcePermissionRequest
        };
        use authios_sdk::responses::{
            LoggedUserCheckServicePermissionResponse as ServicePermissionResponse,
            LoggedUserListResourcePermissionsResponse as ResourcePermissionResponse
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
            .list(ResourcePermissionRequest {
                service_id: String::from("taskios"),
                resource_type: String::from("vault"),
                page_number: *params.page_number,
                get_service_id: false,
                get_resource_type: false,
                get_resource_id: true,
                get_permission_names: true,
                get_page_number: true
            })
            .await;

        let permissions_page = match resource_permission_response {
            ResourcePermissionResponse::Ok { page } => page,
            ResourcePermissionResponse::InvalidToken => return Err(Error::InvalidToken),
            ResourcePermissionResponse::ServerNotAuthios => panic!("AUTH SERVER ERROR: auth server returns invalid responses"),
            ResourcePermissionResponse::ServerUnavailable => panic!("AUTH SERVER ERROR: auth server shut down")
        };
        
        let permissions = match permissions_page.permissions {
            Some(permissions) => permissions,
            None => return Ok(None)
        };

        let mut user_projects = vec![];

        // the page has only five elements
        for permission in permissions.iter() {
            let id = match permission.resource_id.clone().unwrap().parse() {
                Ok(id) => id,
                _ => continue
            };

            let sql = "SELECT name FROM projects WHERE id = $1;";
            let result: Result<(String,), sqlx::Error> = sqlx::query_as(sql)
                .bind(&id)
                .fetch_one(&mut *database_connection)
                .await;

            let name = match result {
                Ok(row) => row.0,
                _ => continue
            };

            let permissions = permission.permissions
                .clone()
                .unwrap();

            let user_project = UserProject {
                id,
                name,
                permissions
            };

            user_projects.push(user_project);
        }

        return Ok(Some(user_projects));
    }
}
